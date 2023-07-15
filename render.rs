use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Context};
use clap::Parser;

const TEMPLATE: &str = include_str!("docker/Dockerfile");

/// Renders a Dockerfile at `docker/Docker.<target>`.
///
/// This is a simple tool to render Dockerfiles that will be published as Docker
/// images for use with [`sheldon`]'s CI.
///
/// [`sheldon`]: https://github.com/rossmacarthur/sheldon
#[derive(Debug, Parser)]
struct Opts {
    /// The target triple.
    #[clap(long)]
    target: String,
    /// OpenSSL install args.
    #[clap(long)]
    install_openssl_args: String,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

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
