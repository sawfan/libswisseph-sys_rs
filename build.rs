use std::env;
use std::path::Path;
use std::process::Command;

const LIBSWISSEPH_DIR: &str = "libswisseph";

const CORE_SWISSEPH_C_FILES: &[&str] = &[
    "swedate.c",
    "swehouse.c",
    "swejpl.c",
    "swemmoon.c",
    "swemplan.c",
    "sweph.c",
    "swephlib.c",
    "swecl.c",
    "swehel.c",
];

const WASM_ONLY_C_FILES: &[&str] = &["wasm_shims.c"];

const VFS_C_FILES: &[&str] = &["vfs/swevfs.c", "vfs/swevfs_stdio.c"];

fn wasi_sysroot() -> Option<String> {
    env::var("WASI_SYSROOT").ok().or_else(|| {
        env::var("WASI_SDK_PATH")
            .ok()
            .map(|p| format!("{p}/share/wasi-sysroot"))
    })
}

fn detect_wasi_sysroot_from_compiler(compiler: &Path, clang_target: &str) -> Option<String> {
    let try_run = |args: &[&str]| -> Option<String> {
        let out = Command::new(compiler).args(args).output().ok()?;
        if !out.status.success() {
            return None;
        }

        let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if s.is_empty() {
            return None;
        }

        if Path::new(&s).exists() {
            Some(s)
        } else {
            None
        }
    };

    try_run(&["--print-sysroot"])
        .or_else(|| try_run(&[&format!("--target={clang_target}"), "--print-sysroot"]))
}

fn print_rerun_directives() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed={LIBSWISSEPH_DIR}");

    println!("cargo:rerun-if-env-changed=WASI_SYSROOT");
    println!("cargo:rerun-if-env-changed=WASI_SDK_PATH");
    println!("cargo:rerun-if-env-changed=CC_FORCE_DISABLE");
    println!("cargo:rerun-if-env-changed=CC_ENABLE_DEBUG_OUTPUT");

    println!("cargo:rerun-if-env-changed=CFLAGS");
    println!("cargo:rerun-if-env-changed=CFLAGS_wasm32_unknown_unknown");
    println!("cargo:rerun-if-env-changed=CFLAGS_wasm32_wasip1");
    println!("cargo:rerun-if-env-changed=CFLAGS_wasm32_wasi");
}

fn ensure_libswisseph_checkout() {
    let required_file = Path::new(LIBSWISSEPH_DIR).join("sweph.c");

    if required_file.exists() {
        return;
    }

    let status = Command::new("git")
        .args([
            "submodule",
            "update",
            "--init",
            "--recursive",
            LIBSWISSEPH_DIR,
        ])
        .status();

    match status {
        Ok(s) if s.success() && required_file.exists() => {}
        Ok(s) => {
            panic!(
                "{LIBSWISSEPH_DIR} submodule is missing or incomplete. \
                 Expected {}. \
                 Tried to initialize it, but git exited with status {s}. \
                 Run: git submodule update --init --recursive",
                required_file.display()
            );
        }
        Err(e) => {
            panic!(
                "{LIBSWISSEPH_DIR} submodule is missing or incomplete. \
                 Expected {}. \
                 Could not run git: {e}. \
                 Run: git submodule update --init --recursive",
                required_file.display()
            );
        }
    }
}

fn configure_common_build(build: &mut cc::Build, target: &str) {
    build.warnings(false);
    build.target(target);

    build.include(LIBSWISSEPH_DIR);

    let vfs_include_dir = Path::new(LIBSWISSEPH_DIR).join("vfs");
    if vfs_include_dir.exists() {
        build.include(vfs_include_dir);
    }

    // Match upstream sweph-wasm behavior.
    build.define("USECASE", "2");

    // Avoid dladdr()/dlfcn-dependent library path logic.
    build.define("NO_SWE_GLP", None);

    // Keep file offsets sane on native Unix-style targets.
    build.define("_FILE_OFFSET_BITS", "64");
}

fn configure_wasi_build(build: &mut cc::Build, sysroot: Option<&str>) {
    if let Some(sysroot) = sysroot {
        build.flag(&format!("--sysroot={sysroot}"));
    }
}

fn add_required_c_files(build: &mut cc::Build, dir: impl AsRef<Path>, files: &[&str]) -> usize {
    let dir = dir.as_ref();

    if !dir.exists() {
        panic!("C source directory does not exist: {}", dir.display());
    }

    let mut added = 0usize;

    for file in files {
        let path = dir.join(file);

        if !path.exists() {
            panic!("Required C source file does not exist: {}", path.display());
        }

        build.file(path);
        added += 1;
    }

    added
}

fn add_optional_c_files(build: &mut cc::Build, dir: impl AsRef<Path>, files: &[&str]) -> usize {
    let dir = dir.as_ref();

    if !dir.exists() {
        panic!("C source directory does not exist: {}", dir.display());
    }

    let mut added = 0usize;

    for file in files {
        let path = dir.join(file);

        if path.exists() {
            build.file(path);
            added += 1;
        }
    }

    added
}

fn main() {
    print_rerun_directives();
    ensure_libswisseph_checkout();

    let target = env::var("TARGET").unwrap_or_default();
    let is_wasm_target = target.starts_with("wasm32");
    let is_wasi_target = target.starts_with("wasm32-wasi") || target.starts_with("wasm32-wasip1");

    let feature_vfs = env::var_os("CARGO_FEATURE_VFS").is_some();
    let feature_wasi = env::var_os("CARGO_FEATURE_WASI").is_some();

    let mut cfg = cc::Build::new();
    configure_common_build(&mut cfg, &target);

    let mut added_files = 0usize;

    added_files += add_required_c_files(&mut cfg, LIBSWISSEPH_DIR, CORE_SWISSEPH_C_FILES);

    if is_wasm_target {
        added_files += add_optional_c_files(&mut cfg, LIBSWISSEPH_DIR, WASM_ONLY_C_FILES);
    }

    if feature_vfs || feature_wasi {
        added_files += add_required_c_files(&mut cfg, LIBSWISSEPH_DIR, VFS_C_FILES);
    }

    if is_wasi_target {
        let resolved_wasi_sysroot = wasi_sysroot().or_else(|| {
            let compiler = cc::Build::new().target(&target).get_compiler();
            detect_wasi_sysroot_from_compiler(compiler.path(), &target)
        });

        configure_wasi_build(&mut cfg, resolved_wasi_sysroot.as_deref());
    }

    if added_files == 0 {
        panic!(
            "No C source files were added to the swisseph build. \
             Check that {LIBSWISSEPH_DIR}/ is initialized and contains Swiss Ephemeris C sources."
        );
    }

    cfg.compile("swisseph");

    // libm is not available on WASI or wasm32-unknown-unknown.
    if !target.contains("msvc")
        && !target.starts_with("wasm32-unknown-unknown")
        && !target.starts_with("wasm32-wasi")
        && !target.starts_with("wasm32-wasip1")
    {
        println!("cargo:rustc-link-lib=m");
    }

    println!("cargo:rustc-link-lib=static=swisseph");
}
