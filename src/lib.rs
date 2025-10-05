// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::io::Write;

use clap::Parser;
use color_eyre::Result;
use readme::shields_get;

mod cli;
mod readme;
mod repo;

pub fn md_write_repo<W: std::io::Write>(
    o: &mut std::io::BufWriter<W>,
    repo: &repo::Repo,
) -> Result<()> {
    write!(
        o,
        "| [{}]({}) | {} |",
        repo.name, repo.html_url, repo.stargazers_count
    )?;
    if let Some(readme) = &repo.readme {
        let shields = shields_get(readme).unwrap();
        for shield in shields {
            write!(o, " {}", shield)?;
        }
    }
    writeln!(o, " |")?;
    Ok(())
}

pub fn md_write_repos<'a, W: std::io::Write>(
    o: &mut std::io::BufWriter<W>,
    title: &str,
    repos: impl Iterator<Item = &'a repo::Repo>,
) -> Result<()> {
    let mut first = true;
    for repo in repos {
        if first {
            writeln!(o, "# {title}")?;
            writeln!(o)?;
            writeln!(o, "| Repository | Stars | Shields |")?;
            writeln!(o, "| -- | --: | -- |")?;
            first = false;
        }
        md_write_repo(o, repo)?;
    }
    if !first {
        writeln!(o)?;
    }
    Ok(())
}

pub fn markdown_write<W: std::io::Write>(
    repos: &[repo::Repo],
    mut o: std::io::BufWriter<W>,
) -> Result<()> {
    writeln!(o)?;
    md_write_repos(
        &mut o,
        "Active Repositories",
        repos.iter().filter(|r| r.permissions.admin && !r.archived),
    )?;
    md_write_repos(
        &mut o,
        "Archived Repositories",
        repos.iter().filter(|r| r.permissions.admin && r.archived),
    )?;
    md_write_repos(
        &mut o,
        "Third-party active repositories",
        repos.iter().filter(|r| !r.permissions.admin && !r.archived),
    )?;
    md_write_repos(
        &mut o,
        "Third-party archived repositories",
        repos.iter().filter(|r| !r.permissions.admin && r.archived),
    )?;
    Ok(())
}

#[tokio::main]
pub async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::ACTIVE)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let args = cli::Cli::parse();
    let token = repo::token_get()?;
    let repos = repo::data_get(&token).await?;
    if let Some(output) = args.output {
        let file = std::fs::File::create(output)?;
        let writer = std::io::BufWriter::new(file);
        markdown_write(&repos, writer)?;
    } else {
        let stdout = std::io::stdout();
        let handle = stdout.lock();
        let writer = std::io::BufWriter::new(handle);
        markdown_write(&repos, writer)?;
    }
    Ok(())
}
