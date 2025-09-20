// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use clap::Parser;
use readme::shields_get;

mod cli;
mod readme;
mod repo;

pub fn markdown_write(repos: &[repo::Repo]) {
    println!("# Repositories");
    println!();
    println!("| Repository  | Shields  |");
    println!("| -- | -- |");
    for repo in repos {
        print!("| [{}]({})  |", repo.name, repo.html_url);
        if let Some(readme) = &repo.readme {
            let shields = shields_get(readme).unwrap();
            for shield in shields {
                print!(" ![]({})", shield);
            }
        }
        println!(" |");
    }
}

#[tokio::main]
pub async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::ACTIVE)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let _args = cli::Cli::parse();
    let token = repo::token_get()?;
    let repos = repo::data_get(&token).await?;
    markdown_write(&repos);
    Ok(())
}
