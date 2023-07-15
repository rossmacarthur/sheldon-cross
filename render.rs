use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Context};
use argh::FromArgs;

const TEMPLATE: &str = include_str!("docker/Dockerfile");

/// Renders a Dockerfile at `docker/Docker.<target>`.
#[derive(Debug, FromArgs)]
struct Opts {
    /// the target triple.
    #[argh(option)]
    target: String,
    /// the OpenSSL install args.
    #[argh(option)]
    install_openssl_args: String,
}

fn main() -> anyhow::Result<()> {
    let opts: Opts = argh::from_env();

    let data = upon::value! {
        target: &opts.target,
        is_musl: &opts.target.contains("musl"),
        install_openssl_args: opts.install_openssl_args,
    };

    let rendered = upon::Engine::new()
        .compile(TEMPLATE)
        .map_err(|err| anyhow!("{:?}", err))?
        .render(data)?;

    let path: PathBuf = ["docker", &format!("Dockerfile.{}", opts.target)]
        .iter()
        .collect();

    fs::write(&path, &rendered)
        .with_context(|| format!("failed to write to `{}`", path.display()))?;

    eprintln!("Rendered `{}`\n", path.display());
    println!("{}", rendered);
    Ok(())
}
