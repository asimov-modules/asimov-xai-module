// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "std"))]
compile_error!("asimov-xai-prompter requires the 'std' feature");

use asimov_module::SysexitsError::{self, *};
use asimov_module::tracing;
use clap::Parser;
use clientele::StandardOptions;
use std::{error::Error, io::Read};

/// asimov-xai-prompter
#[derive(Debug, Parser)]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    input: Option<String>,
    output: Option<String>,
}

pub fn main() -> Result<SysexitsError, Box<dyn Error>> {
    // Load environment variables from `.env`:
    asimov_module::dotenv().ok();

    // Expand wildcards and @argfiles:
    let args = asimov_module::args_os()?;

    // Parse command-line options:
    let options = Options::parse_from(args);

    // Handle the `--version` flag:
    if options.flags.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(EX_OK);
    }

    // Handle the `--license` flag:
    if options.flags.license {
        print!("{}", include_str!("../../UNLICENSE"));
        return Ok(EX_OK);
    }

    let Ok(manifest) = asimov_module::ModuleManifest::read_manifest("xai")
        .inspect_err(|e| eprintln!("failed to read module manifest: {e}"))
    else {
        return Ok(EX_CONFIG);
    };

    let Ok(api_key) = manifest
        .variable("api-key", None)
        .inspect_err(|e| eprintln!("failed to read configured API key: {e}"))
    else {
        return Ok(EX_CONFIG); // not configured
    };
    let Ok(endpoint) = manifest
        .variable("endpoint", None)
        .inspect_err(|e| eprintln!("failed to read configured endpoint: {e}"))
    else {
        return Ok(EX_CONFIG); // not configured
    };
    let Ok(model) = manifest
        .variable("model", None)
        .inspect_err(|e| eprintln!("failed to read configured model: {e}"))
    else {
        return Ok(EX_CONFIG); // not configured
    };

    // Configure logging & tracing:
    #[cfg(feature = "tracing")]
    asimov_module::init_tracing_subscriber(&options.flags).expect("failed to initialize logging");

    let input = if let Some(input) = options.input {
        let file = std::path::Path::new(&input);
        std::fs::read_to_string(file)
            .inspect_err(|e| tracing::error!("unable to read input file: {e}"))?
    } else {
        let mut buf = String::new();
        std::io::stdin()
            .read_to_string(&mut buf)
            .inspect_err(|e| tracing::error!("unable to read STDIN: {e}"))?;
        buf.trim().to_string()
    };

    let mut output: Box<dyn std::io::Write> = if let Some(output) = options.output {
        let file = std::path::Path::new(&output);
        let out = std::fs::File::create(file)
            .inspect_err(|e| tracing::error!("unable to open output file: {e}"))?;
        Box::new(out)
    } else {
        let out = std::io::stdout().lock();
        Box::new(out)
    };

    let options = asimov_xai_module::Options::builder()
        .endpoint(endpoint)
        .model(model)
        .api_key(api_key)
        .build();

    let response = asimov_xai_module::generate(&input, &options)?;

    for text in response {
        output.write_all(text.as_bytes()).unwrap();
    }

    Ok(EX_OK)
}
