fn main() {
    use build_script_cfg::Cfg;
    use search_infini_tools::{extra_config, find_infer_cc, get_or_set_infini_root};
    use std::process::Command;
    use std::{env, path::PathBuf};

    let cfg = Cfg::new("infini");
    let Some(src) = find_infer_cc() else {
        return;
    };

    let lib = get_or_set_infini_root().join("lib");
    let extra_config = extra_config();

    if !lib.read_dir().map_or(false, |dir| {
        dir.filter_map(|entry| entry.ok())
            .filter_map(|entry| entry.file_name().into_string().ok())
            .any(|name| name.contains("infinirt"))
    }) {
        let xmake = || {
            let mut cmd = Command::new("xmake");
            cmd.current_dir(&src);
            cmd
        };

        let mut config = xmake();
        config.args(["f", "--ccl=false", "--infer=false"]);
        if !extra_config.is_empty() {
            config.arg(extra_config);
        }
        assert!(config.status().unwrap().success());
        assert!(xmake().status().unwrap().success());
        assert!(xmake().arg("install").status().unwrap().success());
    }

    cfg.define();
    println!("cargo:rustc-link-search={}", lib.display());
    println!("cargo::rustc-link-arg=-Wl,-rpath,{}", lib.display());
    println!("cargo:rustc-link-lib=infinirt");

    // The bindgen::Builder is the main entry point to bindgen,
    // and lets you build up options for the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header("wrapper.h")
        .clang_arg(format!("-I{}", src.join("include").display()))
        // Only generate bindings for the functions in these namespaces.
        .allowlist_item("infinirt.*")
        .allowlist_item("DeviceType")
        // Annotate the given type with the #[must_use] attribute.
        // Nothing...
        // Generate rust style enums.
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        })
        // Use core instead of std in the generated bindings.
        .use_core()
        // Tell cargo to invalidate the built crate whenever any of the included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
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
