use std::{
    collections::HashSet,
    env, fs, io,
    path::{Path, PathBuf},
};

const OPTIONALS_DIR: &str = "src/optionals";
const COMPILER_DIR: &str = "src/compiler";
const RUNTIME_DIR: &str = "src/runtime";
const SHARED_DIR: &str = "src/shared";
const UTILS_DIR: &str = "src/utils";

#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}

fn add_c_files<P>(dir: P, builder: &mut cc::Build) -> io::Result<()>
where
    P: AsRef<Path>,
{
    if dir.as_ref().is_dir() {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "c" {
                        println!("cargo:rerun-if-changed={}", path.display());
                        builder.file(path);
                    } else if extension == "h" {
                        println!("cargo:rerun-if-changed={}", path.display());
                    }
                }
            }
        }
    }
    Ok(())
}

fn main() {
    let mut builder = cc::Build::new();

    add_c_files(OPTIONALS_DIR, &mut builder).unwrap();
    add_c_files(COMPILER_DIR, &mut builder).unwrap();
    add_c_files(RUNTIME_DIR, &mut builder).unwrap();
    add_c_files(SHARED_DIR, &mut builder).unwrap();
    add_c_files(UTILS_DIR, &mut builder).unwrap();

    builder
        .flag("-std=gnu99")
        .flag("-fgnu89-inline")
        .pic(true)
        .warnings(false)
        .define("BUILD_GRAVITY_API", None)
        .include(OPTIONALS_DIR)
        .include(COMPILER_DIR)
        .include(RUNTIME_DIR)
        .include(SHARED_DIR)
        .include(UTILS_DIR)
        .compile("gravety.a");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // Macros that cause dup errors, so we ignore them.
    // See: https://github.com/rust-lang/rust-bindgen/issues/687#issuecomment-450750547
    let ignored_macros = IgnoreMacros(
        vec![
            "FP_INFINITE".into(),
            "FP_NAN".into(),
            "FP_NORMAL".into(),
            "FP_SUBNORMAL".into(),
            "FP_ZERO".into(),
            "IPPORT_RESERVED".into(),
        ]
        .into_iter()
        .collect(),
    );

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .clang_args(&[
            format!("-I{}/", OPTIONALS_DIR),
            format!("-I{}/", COMPILER_DIR),
            format!("-I{}/", RUNTIME_DIR),
            format!("-I{}/", SHARED_DIR),
            format!("-I{}/", UTILS_DIR),
        ])
        // // Whitelist only functions the start with a prefix.
        // .whitelist_function("(gravity|json|token|gnode|ircode)_.*")
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(ignored_macros))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
