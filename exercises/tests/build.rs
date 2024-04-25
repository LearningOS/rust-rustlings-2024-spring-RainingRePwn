fn main() {
    // 设置名为 TEST_FOO 的环境变量，并将其值设置为当前时间戳
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 设置名为 pass 的特性
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
