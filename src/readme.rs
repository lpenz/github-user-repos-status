// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use color_eyre::Result;
use pulldown_cmark::{Event, Parser, Tag};

pub fn shields_get(string: &str) -> Result<Vec<String>> {
    let mut shields = Vec::<String>::default();
    let parser = Parser::new(string);
    for event in parser {
        match event {
            Event::Start(Tag::Image { dest_url, .. }) => {
                shields.push(dest_url.to_string());
            }
            Event::Start(Tag::Heading { .. }) => {
                break;
            }
            _ => {}
        }
    }
    Ok(shields)
}
