use std::{fs, path::PathBuf};

use anyhow::Context;
use handlebars::Handlebars;
use serde_json::json;
use structopt::StructOpt;

const TEMPLATE: &str = include_str!("Dockerfile");

/// Renders a Dockerfile at `docker/Docker.<target>`.
///
/// This is a simple tool to render Dockerfiles that will be published as Docker
/// images for use with [`sheldon`]'s CI.
///
/// [`sheldon`]: https://github.com/rossmacarthur/sheldon
#[derive(Debug, StructOpt)]
struct Opt {
    /// The target triple.
    #[structopt(long)]
    target: String,
    /// OpenSSL install args.
    #[structopt(long)]
    install_openssl_args: String,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);
    handlebars
        .register_template_string("dockerfile", TEMPLATE)
        .context("failed to register template")?;
    let path: PathBuf = ["docker", &format!("Dockerfile.{}", opt.target)]
        .iter()
        .collect();
    let data = json!({
        "target": opt.target,
        "install_gnupg": opt.target.contains("musl"),
        "install_openssl_args": opt.install_openssl_args,
    });
    let rendered = handlebars
        .render("dockerfile", &data)
        .context("failed to render Dockerfile")?;
    fs::write(&path, rendered)
        .with_context(|| format!("failed to write to `{}`", path.display()))?;
    println!("Rendered `{}`", path.display());
    Ok(())
}
