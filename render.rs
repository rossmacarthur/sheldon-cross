use std::fs;
use std::path::PathBuf;

use anyhow::Context;
use clap::Clap;
use handlebars::Handlebars;
use serde_json::json;

const TEMPLATE: &str = include_str!("docker/Dockerfile");

/// Renders a Dockerfile at `docker/Docker.<target>`.
///
/// This is a simple tool to render Dockerfiles that will be published as Docker
/// images for use with [`sheldon`]'s CI.
///
/// [`sheldon`]: https://github.com/rossmacarthur/sheldon
#[derive(Debug, Clap)]
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
    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);
    handlebars
        .register_template_string("dockerfile", TEMPLATE)
        .context("failed to register template")?;
    let path: PathBuf = ["docker", &format!("Dockerfile.{}", opts.target)]
        .iter()
        .collect();
    let data = json!({
        "target": opts.target,
        "is_musl": opts.target.contains("musl"),
        "install_openssl_args": opts.install_openssl_args,
    });
    let rendered = handlebars
        .render("dockerfile", &data)
        .context("failed to render Dockerfile")?;
    fs::write(&path, &rendered)
        .with_context(|| format!("failed to write to `{}`", path.display()))?;
    eprintln!("Rendered `{}`\n", path.display());
    println!("{}", rendered);
    Ok(())
}
