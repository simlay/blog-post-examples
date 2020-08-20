use std::env;
use std::path::Path;
fn main() {

    let target = std::env::var("TARGET").unwrap();
    let target = if target == "aarch64-apple-ios" {
        "arm64-apple-ios"
    } else {
        &target
    };

    let sdk_path = "/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS13.6.sdk";

    let builder = bindgen::Builder::default()
        .rustfmt_bindings(true)
        .header_contents("UIKit.h", "#include<UIKit/UIKit.h>")

        .clang_args(&[&format!("--target={}", target)])
        .clang_args(&["-isysroot", sdk_path])

        .block_extern_crate(true)
        .generate_block(true)
        .clang_args(&["-fblocks"])

        .objc_extern_crate(true)
        .clang_args(&["-x", "objective-c"])

        .blacklist_item("timezone")
        .blacklist_item("IUIStepper")
        .blacklist_function("dividerImageForLeftSegmentState_rightSegmentState_")
        .blacklist_item("objc_object")
        ;


    println!("cargo:rustc-link-lib=framework=UIKit");
    let bindings = builder.generate().expect("unable to generate bindings");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    bindings
        .write_to_file(Path::new(&out_dir).join("uikit.rs"))
        .expect("could not write bindings");
}
