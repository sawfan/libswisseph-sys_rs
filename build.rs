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
    //cfg.warnings(false);
    add_c_files(&mut cfg, "libswisseph");
    cfg.compile("swisseph");

    let macros = Arc::new(RwLock::new(HashSet::new()));
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(MacroCallback {macros: macros.clone()}))
        .generate()
        .expect("Unable to generate bindings");


    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
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


//    cfg.include(&include)
//        .include("libgit2/src/libgit2")
//        .include("libgit2/src/util")
//        .out_dir(dst.join("build"))
//        .warnings(false);

    //add_h_files(&mut cfg, "libswisseph");
    //cfg.flag("-g");
    //cfg.flag("-Wall");
//
//
//
//
//
//
//
   //println!("cargo:rustc-link-lib=foo");
    //
   //let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
   //let include = dst.join("include");

   //fs::create_dir_all(&include).unwrap();

   // Copy over all header files
   //cp_r("libswisseph", &include);

//    cfg.include(&include)
//     //  .include("libswisseph")
//        .out_dir(dst.join("build"))
//       // .warnings(true); // Turns on -Wall, enabled by default
//        .warnings(false); // Turns on -Wall, enabled by default
    //
    //cfg.include(&include);
    //cfg.out_dir(dst.join("build"));
//
//
//
    // BIND STUFF BELOW

    // Tell cargo to look for shared libraries in the specified directory
    //println!("cargo:rustc-link-search=/path/to/lib");
    //println!("cargo:rustc-link-search=/home/yoni/Code/personal/libswisseph-sys/libswisseph");


    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    //println!("cargo:rustc-link-lib=swisseph");

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
