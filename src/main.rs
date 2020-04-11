//! This is a simple tool to do the following things render Dockerfiles and
//! publish Docker images for use with sheldon's CI.

use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

use anyhow::Context;
use handlebars::Handlebars;
use serde_json::json;

const TEMPLATE: &str = include_str!("Dockerfile");

const TARGETS: &[Target] = &[
    Target {
        triple: "x86_64-unknown-linux-gnu",
        install_openssl_args: &["linux-x86_64"],
    },
    Target {
        triple: "x86_64-unknown-linux-musl",
        install_openssl_args: &["linux-x86_64", "x86_64-linux-musl-"],
    },
    Target {
        triple: "armv7-unknown-linux-gnueabihf",
        install_openssl_args: &["linux-armv4", "arm-linux-gnueabihf-"],
    },
    Target {
        triple: "armv7-unknown-linux-musleabihf",
        install_openssl_args: &["linux-armv4", "arm-linux-musleabihf-"],
    },
];

#[derive(Debug)]
struct Target {
    triple: &'static str,
    install_openssl_args: &'static [&'static str],
}

fn write(path: &Path, rendered: String, bless: bool) -> anyhow::Result<()> {
    match fs::read_to_string(&path) {
        Ok(current) => {
            if current == rendered {
                println!("Checked `{}`", path.display());
                return Ok(());
            } else if !bless {
                anyhow::bail!(
                    "`{}` differs, run with `--bless` to overwrite",
                    path.display()
                );
            }
        }
        Err(err) => {
            if err.kind() != io::ErrorKind::NotFound {
                return Err(err)
                    .with_context(|| format!("failed to read from `{}`", path.display()));
            }
        }
    }
    fs::write(&path, rendered)
        .with_context(|| format!("failed to write to `{}`", path.display()))?;
    println!("Rendered `{}`", path.display());
    Ok(())
}

fn write_dockerfiles(bless: bool) -> anyhow::Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);
    handlebars
        .register_template_string("dockerfile", TEMPLATE)
        .context("failed to register template")?;
    for target in TARGETS {
        let file = format!("Dockerfile.{}", target.triple);
        let path: PathBuf = ["docker", &file].iter().collect();
        let data = json!({
            "triple": target.triple,
            "install_gnupg": target.triple.contains("musl"),
            "install_openssl_args": target.install_openssl_args.join(" "),
        });
        let rendered = handlebars
            .render("dockerfile", &data)
            .context("failed to render Dockerfile")?;
        write(&path, rendered, bless)?;
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let bless = match &env::args().nth(1).as_ref().map(|s| &s[..]) {
        Some("--bless") => true,
        None => false,
        Some(_) => anyhow::bail!("unexpected command line argument"),
    };
    write_dockerfiles(bless)?;
    Ok(())
}
