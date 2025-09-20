// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::env;

use color_eyre::Result;
use color_eyre::eyre::WrapErr;
use derive_more::Display;
use futures::future::join_all;
use reqwest::Client;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Display)]
pub struct Token(String);

#[derive(Debug, Deserialize)]
pub struct Repo {
    pub name: String,
    pub html_url: String,
    pub private: bool,
    pub archived: bool,
    pub owner: Owner,
    pub readme: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Owner {
    pub login: String,
}

pub fn token_get() -> Result<Token> {
    let token_str =
        env::var("GITHUB_USER_REPOS_STATUS").wrap_err("GITHUB_USER_REPOS_STATUS not found")?;
    Ok(Token(token_str))
}

pub async fn list_get(token: &Token) -> Result<Vec<Repo>> {
    let client = Client::new();
    Ok(client
        .get("https://api.github.com/user/repos?per_page=100")
        .header("Authorization", format!("token {}", token))
        .header("User-Agent", "rust-reqwest")
        .send()
        .await?
        .json()
        .await?)
}

/// Returns the contents of the readme file of the given repository
pub async fn readme_get(token: &Token, repo: &Repo) -> Result<String> {
    let client = Client::new();
    let url = format!(
        "https://api.github.com/repos/{}/{}/readme",
        repo.owner.login, repo.name
    );
    #[derive(Debug, Deserialize)]
    pub struct ReadmeInfo {
        pub download_url: Url,
    }
    let readme: ReadmeInfo = client
        .get(&url)
        .header("Authorization", format!("token {}", token))
        .header("User-Agent", "rust-reqwest")
        .send()
        .await?
        .json()
        .await?;
    let content = client
        .get(readme.download_url)
        .header("User-Agent", "rust-reqwest")
        .send()
        .await?
        .text()
        .await?;
    Ok(content)
}

/// Gets all data from all repositories, including the readme contents
pub async fn data_get(token: &Token) -> Result<Vec<Repo>> {
    let repos = list_get(token).await?;
    let futures = repos.into_iter().map(async |mut repo| {
        let readme = readme_get(token, &repo)
            .await
            .wrap_err_with(|| format!("while processing {}", repo.name))?;
        repo.readme = Some(readme);
        Ok(repo)
    });
    let results = join_all(futures).await;
    results.into_iter().collect()
}
