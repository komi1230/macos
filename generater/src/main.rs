use bindgen;
use std::env;
use std::path::Path;
use std::{fs, io};

fn main() -> io::Result<()> {
    let target = "x86_64-apple-darwin";

    let sdk_version = "MacOSX11.1.sdk";

    let sdk_path = Path::new("/Library/Developer/CommandLineTools/SDKs").join(sdk_version);

    let project_root = env::current_dir()?;

    println!("cargo:rustc-link-lib=framework=Foundation");

    let frameworks_dir = sdk_path.join("System/Library/Frameworks");
    for framework_path in fs::read_dir(frameworks_dir)?.map(|p| p.unwrap().path()) {
        let framework_name = framework_path.file_name().unwrap().to_str().unwrap();
        let framework_header = framework_name.replace(".framework", ".h");

        println!("{:?}", framework_name);

        if framework_name != "Foundation.framework" {
            continue;
        }

        // Generate bindings
        let tmp = framework_path.join("Headers").join(framework_header);
        let header_file = tmp.to_str().unwrap();

        let builder = bindgen::Builder::default()
            .rustfmt_bindings(true)
            .header(header_file)
            .clang_args(&[&format!("--target={}", target)])
            .clang_args(&["-isysroot", sdk_path.to_str().unwrap()])
            .block_extern_crate(true)
            .generate_block(true)
            .clang_args(&["-fblocks"])
            .objc_extern_crate(true)
            .clang_args(&["-x", "objective-c"])
            .blocklist_item("timezone")
            .blocklist_item("IUIStepper")
            .blocklist_function("dividerImageForLeftSegmentState_rightSegmentState_")
            .blocklist_item("objc_object");

        let bindings = builder.generate().expect("unable to generate bindings");

        let rust_file_name = framework_name.replace(".framework", ".rs");
        let saved_file_path = project_root.join("result").join(rust_file_name);
        println!("{:?}", saved_file_path);
        fs::File::create(&saved_file_path)?;

        bindings
            .write_to_file(saved_file_path)
            .expect("Failed to save");
    }

    Ok(())
}
