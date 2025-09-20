// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use clap::Parser;
use std::env;

mod cli;
mod repo;

#[tokio::main]
pub async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::ACTIVE)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let _args = cli::Cli::parse();
    let token = env::var("GITHUB_USER_REPOS_STATUS").expect("GITHUB_USER_REPOS_STATUS not set");
    let repos = repo::list_get(&token).await?;
    for repo in repos {
        println!(
            "Repo: {:<30} | Private: {:<5} | Arhived: {:<5} | onwer: {} | URL: {}",
            repo.name, repo.private, repo.archived, repo.owner.login, repo.html_url
        );
        let readme = repo::readme_get(&token, &repo).await?;
        println!("    readme url {}", readme);
    }
    Ok(())
}
