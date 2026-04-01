use std::env;
use std::io;
use std::path::Path;
use std::process::Command;

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

fn configure_wasi_build(build: &mut cc::Build, sysroot: Option<&str>) {
    if let Some(sysroot) = sysroot {
        build.flag(&format!("--sysroot={sysroot}"));
    }

    // WASI does not provide dladdr()/dlfcn, and we do not need swe_get_library_path().
    build.define("NO_SWE_GLP", None);
    build.define("_FILE_OFFSET_BITS", "64");
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=libswisseph");
    println!("cargo:rerun-if-env-changed=WASI_SYSROOT");
    println!("cargo:rerun-if-env-changed=WASI_SDK_PATH");

    if !Path::new("libswisseph").exists() {
        let status = Command::new("git")
            .args(["submodule", "update", "--init", "libswisseph"])
            .status();

        match status {
            Ok(s) if s.success() => {}
            Ok(_) | Err(_) => {
                panic!("libswisseph submodule is missing and could not be initialized");
            }
        }
    }

    let target = env::var("TARGET").unwrap_or_default();
    let clang_target = target.clone();

    let mut cfg = cc::Build::new();
    cfg.warnings(false);
    cfg.target(&clang_target);

    // Match upstream sweph-wasm behavior.
    cfg.define("USECASE", "2");

    let is_wasm_target = target.starts_with("wasm32");
    let is_wasi_target = target.starts_with("wasm32-wasi") || target.starts_with("wasm32-wasip1");

    let resolved_wasi_sysroot = if is_wasi_target {
        wasi_sysroot().or_else(|| {
            let compiler = cc::Build::new().target(&clang_target).get_compiler();
            detect_wasi_sysroot_from_compiler(compiler.path(), &clang_target)
        })
    } else {
        None
    };

    if is_wasm_target {
        add_c_files_from_list(
            &mut cfg,
            "libswisseph",
            &[
                "swedate.c",
                "swehouse.c",
                "swejpl.c",
                "swemmoon.c",
                "swemplan.c",
                "sweph.c",
                "swephlib.c",
                "swecl.c",
                "swehel.c",
            ],
        );
    } else {
        add_c_files(&mut cfg, "libswisseph");
    }

    if is_wasi_target {
        configure_wasi_build(&mut cfg, resolved_wasi_sysroot.as_deref());
    }

    cfg.compile("swisseph");

    if !target.contains("msvc") && !target.starts_with("wasm32-unknown-unknown") {
        println!("cargo:rustc-link-lib=m");
    }

    println!("cargo:rustc-link-lib=static=swisseph");
}

fn add_c_files(build: &mut cc::Build, path: impl AsRef<Path>) {
    let path = path.as_ref();
    if !path.exists() {
        panic!("Path {} does not exist", path.display());
    }

    let dir = path.read_dir().unwrap();
    let mut paths = dir.collect::<io::Result<Vec<_>>>().unwrap();
    paths.sort_by_key(|e| e.path());

    for e in paths {
        let path = e.path();
        if e.file_type().unwrap().is_dir() {
            continue;
        }

        if path.extension().and_then(|s| s.to_str()) == Some("c") {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                let exclude_because_has_main = [
                    "sweasp",
                    "swetest",
                    "swevents",
                    "swephgen4",
                    "swemini",
                    "sweephe4",
                ];

                if !exclude_because_has_main.contains(&stem) {
                    build.file(&path);
                }
            }
        }
    }
}

fn add_c_files_from_list(build: &mut cc::Build, dir: impl AsRef<Path>, files: &[&str]) {
    let dir = dir.as_ref();
    if !dir.exists() {
        panic!("Path {} does not exist", dir.display());
    }

    for file in files {
        build.file(dir.join(file));
    }
}
