use cmake::Config;

fn main() {
    fn parse_env(key: &str, default: bool) -> bool {
        use std::env::{var, VarError};

        match var(key) {
            Ok(val) => {
                match &val as &str {
                    "0" => false,
                    "1" => true,
                    _ => default
                }
            },
            Err(VarError::NotPresent) => default,
            Err(VarError::NotUnicode(_)) => panic!("Environment variable is not unicode: {}", key),
        }
    }

    let defensive = parse_env("XM_DEFENSIVE", true);
    let strings = parse_env("XM_STRINGS", true);
    let libxmize_delta_samples = parse_env("XM_LIBXMIZE_DELTA_SAMPLES", true);
    let linear_interpolation = parse_env("XM_LINEAR_INTERPOLATION", true);
    let ramping = parse_env("XM_RAMPING", true);
    let debug = parse_env("XM_DEBUG", false);
    let big_endian = parse_env("XM_BIG_ENDIAN", false);

    fn on_off(value: bool) -> Option<&'static str> {
        Some(if value { "1" } else { "0" })
    }

    let mut config = Config::new("libxm/src");
    config.build_target("xm")
        .define("XM_VERBOSE", "0");
    let dst = config.build();
    let profile = config.get_profile();
    println!("cargo:rustc-link-search=native={}/build/{profile}", dst.display());
    println!("cargo:rustc-link-lib=static=xm");
}
