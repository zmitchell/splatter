//! A simple little script that attempts to run every example listed within the `Cargo.toml` of
//! each specified package. E.g.
//!
//! ```ignore
//! cargo run -p run_all_examples -- nature_of_code
//! ```
//!
//! If no package is specified, all examples for the `examples`, `generative_design` and
//! `nature_of_code` packages will be run. E.g.
//!
//! ```ignore
//! cargo run -p run_all_examples
//! ```
//!
//! TODO: Collect a map of failed examples so that they can be concisely reported at the end.

use anyhow::{bail, Context};
use std::process::Stdio;
use toml_edit::Document;

type Error = anyhow::Error;

fn main() -> Result<(), Error> {
    const ALL_PACKAGES: &[&str] = &["examples", "generative_design", "nature_of_code"];

    // Retrieve the specified packages if any, otherwise default to ALL_PACKAGES.
    let mut args = std::env::args();
    args.next().unwrap();
    let specified_packages: Vec<String> = args.collect();
    let packages: Vec<String> = if specified_packages.is_empty() {
        ALL_PACKAGES.iter().cloned().map(Into::into).collect()
    } else {
        specified_packages
    };

    // Read the splatter cargo manifest to a `toml::Value`.
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let workspace_manifest_dir = std::path::Path::new(manifest_dir)
        .parent()
        .unwrap() // splatter/scripts
        .parent()
        .unwrap(); // splatter

    let mut failures = Vec::new();
    for package in packages {
        let examples_dir = workspace_manifest_dir.join(&package);
        let manifest_path = examples_dir.join("Cargo").with_extension("toml");
        let contents = std::fs::read_to_string(manifest_path)?;
        let toml = contents
            .parse::<Document>()
            .context("contents weren't a valid manifest")?;

        // Frist, build all examples in the package.
        println!("Building examples in splatter/{}...", package);
        let output = std::process::Command::new("cargo")
            .arg("build")
            .arg("-p")
            .arg(&package)
            .arg("--examples")
            .output()
            .context("failed to run `cargo build -p {package} --examples`")?;
        if !output.stderr.is_empty() {
            let stderr =
                String::from_utf8(output.stderr).context("couldn't convert stderr to string")?;
            if stderr.contains("error[E") {
                eprintln!(
                    "failed to build examples for package '{}':\n{}",
                    package, stderr
                );
            }
        }

        // Find the `examples` table within the `toml::Value` to find all example names.
        let examples = toml["example"]
            .as_array_of_tables()
            .context("failed to retrieve example array")?;

        println!("Running examples in splatter/{}...", package);
        for example in examples {
            let name = example["name"]
                .as_str()
                .context("failed to retrieve example name")?;

            // For each example, invoke a cargo sub-process to run the example.
            let mut child = std::process::Command::new("cargo")
                .arg("run")
                .arg("-p")
                .arg(&package)
                .arg("--example")
                .arg(name)
                .stdin(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .context("failed to spawn example '{name}'")?;

            // Allow each example to run for 3 secs each.
            std::thread::sleep(std::time::Duration::from_secs(3));

            // Kill the example and retrieve any output.
            child.kill().ok();
            let output = child
                .wait_with_output()
                .context("failed to wait for child process")?;

            // If the example wrote to `stderr` it must have failed.
            if !output.stderr.is_empty() || output.status.code().unwrap_or(0) == 101 {
                failures.push(name.to_string());
            }
        }
    }
    println!("Failed examples:");
    for ex in failures.iter() {
        println!("{ex}");
    }
    if failures.is_empty() {
        Ok(())
    } else {
        bail!("some examples didn't build properly")
    }
}
