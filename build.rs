use std::env;
use std::error::Error;
use std::fmt::Write;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // This is default if no rerun-if-changed lines in build.rs.
    println!("cargo:rerun-if-changed=src");

    // Make sure we're only building for one device.
    let mutex_features = vec!["msp430g2553", "msp430g2211"];
    let device = match mutex_features
        .iter()
        .filter_map(|t| {
            env::var(format!(
                "CARGO_FEATURE_{}",
                t.to_uppercase().replace('-', "_")
            ))
            .ok()
            .map(|_| (*t).to_owned())
        })
        .collect::<Vec<_>>()
    {
        v if v.len() != 1 => {
            let mut err_str = String::new();
            write!(
                &mut err_str,
                "exactly one of the following features must be set: {}",
                mutex_features.join(", ")
            )?;
            return Err(err_str.into());
        }
        v => v[0].clone(),
    };

    // Find the appropriate memory script and copy to `out`. For examples.
    if env::var("CARGO_FEATURE_RT").is_ok() {
        fs::copy(
            [
                env::var("CARGO_MANIFEST_DIR")?,
                "memory".into(),
                format!("{}.x", device),
            ]
            .iter()
            .collect::<PathBuf>(),
            out.join("memory.x"),
        )?;

        // Tell Rust where to find the memory script, rebuild when script changes.
        println!("cargo:rustc-link-search={}", out.display());
        println!("cargo:rerun-if-changed=memory/{}.x", device);
    }

    Ok(())
}
