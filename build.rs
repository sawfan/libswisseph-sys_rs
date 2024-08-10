use std::env;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::collections::HashSet;
use std::process::Command;

use bindgen::callbacks::{ MacroParsingBehavior, ParseCallbacks };

fn main() {
    if !Path::new("swisseph").exists() {
        let _ = Command::new("git")
            .args(&["submodule", "update", "--init", "libswisseph"])
            .status();
    }

    let mut cfg = cc::Build::new();
    cfg.warnings(false);
    add_c_files(&mut cfg, "libswisseph");
    cfg.compile("swisseph");

    let macros = Arc::new(RwLock::new(HashSet::new()));
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .blocklist_function("__.*")
        .blocklist_function("scalbl")
        .blocklist_function("fmal")
        .blocklist_function("fminl")
        .blocklist_function("fmaxl")
        .blocklist_function("fdiml")
        .blocklist_function("llroundl")
        .blocklist_function("llrintl")
        .blocklist_function("remquol")
        .blocklist_function("truncl")
        .blocklist_function("roundl")
        .blocklist_function("nearbyintl")
        .blocklist_function("scalblnl")
        .blocklist_function("scalbnl")
        .blocklist_function("ilogbl")
        .blocklist_function("lroundl")
        .blocklist_function("lrintl")
        .blocklist_function("remainderl")
        .blocklist_function("nexttowardl")
        .blocklist_function("nextafterl")
        .blocklist_function("rintl")
        .blocklist_function("lgammal_r")
        .blocklist_function("gammal")
        .blocklist_function("tgammal")
        .blocklist_function("lgammal")
        .blocklist_function("erfcl")
        .blocklist_function("erfl")
        .blocklist_function("ynl")
        .blocklist_function("y1l")
        .blocklist_function("y0l")
        .blocklist_function("jnl")
        .blocklist_function("j1l")
        .blocklist_function("j0l")
        .blocklist_function("isnanl")
        .blocklist_function("nanl")
        .blocklist_function("copysignl")
        .blocklist_function("copysignl")
        .blocklist_function("significandl")
        .blocklist_function("dreml")
        .blocklist_function("finitel")
        .blocklist_function("isinfl")
        .blocklist_function("fmodl")
        .blocklist_function("floorl")
        .blocklist_function("fabsl")
        .blocklist_function("ceill")
        .blocklist_function("cbrtl")
        .blocklist_function("hypotl")
        .blocklist_function("sqrtl")
        .blocklist_function("powl")
        .blocklist_function("log2l")
        .blocklist_function("exp2l")
        .blocklist_function("logbl")
        .blocklist_function("log1pl")
        .blocklist_function("expm1l")
        .blocklist_function("modfl")
        .blocklist_function("log10l")
        .blocklist_function("logl")
        .blocklist_function("ldexpl")
        .blocklist_function("frexpl")
        .blocklist_function("expl")
        .blocklist_function("atanhl")
        .blocklist_function("asinhl")
        .blocklist_function("acoshl")
        .blocklist_function("tanhl")
        .blocklist_function("sinhl")
        .blocklist_function("coshl")
        .blocklist_function("tanl")
        .blocklist_function("sinl")
        .blocklist_function("cosl")
        .blocklist_function("atan2l")
        .blocklist_function("atanl")
        .blocklist_function("acosl")
        .blocklist_function("nexttowardf")
        .blocklist_function("asinl")
        .blocklist_function("nexttoward")
        .blocklist_function("strtold")
        .blocklist_function("qecvt_r")
        .blocklist_function("qfcvt_r")
        .blocklist_function("qgcvt")
        .blocklist_function("ecvt_r")
        .blocklist_function("qfcvt")
        .blocklist_function("qecvt")
        .blocklist_function("fcvt_r")
        .blocklist_function("wctomb")
        .blocklist_function("mblen")

        .parse_callbacks(Box::new(MacroCallback {macros: macros.clone()}))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");


    //println!("cargo:rustc-link-search=libswisseph");
    //println!("cargo:rustc-link-lib=swe");
    //println!("cargo:rustc-link-lib=swevents");
   
    println!("cargo:rustc-link-lib=swisseph");
}

fn add_c_files(build: &mut cc::Build, path: impl AsRef<Path>) {
    let path = path.as_ref();
    if !path.exists() {
        panic!("Path {} does not exist", path.display());
    }
    // sort the C files to ensure a deterministic build for reproducible builds
    let dir = path.read_dir().unwrap();
    let mut paths = dir.collect::<io::Result<Vec<_>>>().unwrap();
    paths.sort_by_key(|e| e.path());

    for e in paths {
        let path = e.path();
        if e.file_type().unwrap().is_dir() {
            // skip dirs for now
        } else if path.extension().and_then(|s| s.to_str()) == Some("c") {
            if let Some(stem) = path.file_stem() {
                let exclude_because_has_main = vec![
                    "sweasp", "swetest", "swevents", "swephgen4", "swemini"
                ];

                if !exclude_because_has_main.contains(&(stem.to_str().unwrap())) {
                    build.file(&path);
                }
            }
        }
    }
}

#[derive(Debug)]
struct MacroCallback {
    macros: Arc<RwLock<HashSet<String>>>,
}

impl ParseCallbacks for MacroCallback {
    fn will_parse_macro(&self, name: &str) -> MacroParsingBehavior {
        self.macros.write().unwrap().insert(name.into());

        // Ignoring these because errors are thrown about duplicate definitions
        if name == "FP_NORMAL" 
        || name == "FP_SUBNORMAL" 
        || name == "FP_ZERO" 
        || name == "FP_NAN" 
        || name == "FP_INFINITE" 
        {
            return MacroParsingBehavior::Ignore;
        }

        MacroParsingBehavior::Default
    }
}


//fn add_h_files(build: &mut cc::Build, path: impl AsRef<Path>) {
//    let path = path.as_ref();
//    if !path.exists() {
//        panic!("Path {} does not exist", path.display());
//    }
//    // sort the C files to ensure a deterministic build for reproducible builds
//    let dir = path.read_dir().unwrap();
//    let mut paths = dir.collect::<io::Result<Vec<_>>>().unwrap();
//    paths.sort_by_key(|e| e.path());
//
//    for e in paths {
//        let path = e.path();
//        if e.file_type().unwrap().is_dir() {
//            // skip dirs for now
//        } else if path.extension().and_then(|s| s.to_str()) == Some("h") {
//            build.include(&path);
//        }
//    }
//}
//
//
//
//
//fn cp_r(from: impl AsRef<Path>, to: impl AsRef<Path>) {
//    for e in from.as_ref().read_dir().unwrap() {
//        let e = e.unwrap();
//        let from = e.path();
//        let to = to.as_ref().join(e.file_name());
//        if e.file_type().unwrap().is_dir() {
//            //fs::create_dir_all(&to).unwrap();
//            //cp_r(&from, &to);
//        } else {
//            // only copy .h files
//            if from.extension().and_then(|s| s.to_str()) == Some("h") {
//                println!("{} => {}", from.display(), to.display());
//                fs::copy(&from, &to).unwrap();
//            }
//
//        }
//    }
//}
//
//
//    //add_h_files(&mut cfg, "libswisseph");
//
//
//    //
//   //let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
//   //let include = dst.join("include");
//
//   //fs::create_dir_all(&include).unwrap();
//
//   // Copy over all header files
//   //cp_r("libswisseph", &include);
//
//    cfg.include(&include)
//     //  .include("libswisseph")
//        .out_dir(dst.join("build"))
//       // .warnings(true); // Turns on -Wall, enabled by default
//        .warnings(false); // Turns on -Wall, enabled by default
//    //
//    //cfg.include(&include);
//    //cfg.out_dir(dst.join("build"));
//
//
//
//
//    //println!("cargo:rustc-link-lib=swisseph");
//
