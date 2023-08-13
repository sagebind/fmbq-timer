use std::env;

fn main() {
    set_build_time();

    let target = env::var("TARGET").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();


    if target_os == "android" {
        println!("cargo:rerun-if-env-changed=ANDROID_NDK_HOME");

        if let Ok(ndk_home) = env::var("ANDROID_NDK_HOME") {
            println!(
                "cargo:rustc-link-search={}/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/{}/30",
                ndk_home,
                target.replace("armv7-linux-androideabi", "arm-linux-androideabi")
            );
        }
    }
}

fn set_build_time() {
    let now = time::OffsetDateTime::now_local().unwrap();
    let format = time::format_description::parse("[year][month][day][hour][minute]").unwrap();

    println!("cargo:rustc-env=BUILD_TIME={}", now.format(&format).unwrap());
}
