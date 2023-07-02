use std::env;

fn main() {
    shadow_rs::new().unwrap();

    let target = env::var("TARGET").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    println!("cargo:rustc-env=BUILD_TIME=VALUE");

    if target_os == "android" {
        println!("cargo:rerun-if-env-changed=ANDROID_HOME");

        let ndk_home = env::var("NDK_HOME").unwrap();
        println!(
            "cargo:rustc-link-search={}/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/{}/30",
            ndk_home,
            target.replace("armv7-linux-androideabi", "arm-linux-androideabi")
        );
    }
}
