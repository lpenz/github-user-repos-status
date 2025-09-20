// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use color_eyre::Result;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Repo {
    pub name: String,
    pub html_url: String,
    pub private: bool,
    pub archived: bool,
    pub owner: Owner,
}

#[derive(Debug, Deserialize)]
pub struct Owner {
    pub login: String,
}

#[derive(Debug, Deserialize)]
pub struct ReadmeInfo {
    pub download_url: String,
}

pub async fn list_get(token: &str) -> Result<Vec<Repo>> {
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

pub async fn readme_get(token: &str, repo: &Repo) -> Result<String> {
    let client = Client::new();
    let url = format!(
        "https://api.github.com/repos/{}/{}/readme",
        repo.owner.login, repo.name
    );
    let readme: ReadmeInfo = client
        .get(&url)
        .header("Authorization", format!("token {}", token))
        .header("User-Agent", "rust-reqwest")
        .send()
        .await?
        .json()
        .await?;
    let content = client
        .get(&readme.download_url)
        .header("User-Agent", "rust-reqwest")
        .send()
        .await?
        .text()
        .await?;
    Ok(content)
}
