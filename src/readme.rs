// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use color_eyre::Result;
use pulldown_cmark::{Event, Parser, Tag, TagEnd};

pub fn shields_get(string: &str) -> Result<Vec<String>> {
    let mut shields = Vec::<String>::default();
    let parser = Parser::new(string);
    let mut link = Option::<String>::default();
    for event in parser {
        match event {
            Event::Start(Tag::Link { ref dest_url, .. }) => {
                link = Some(dest_url.to_string());
            }
            Event::End(TagEnd::Link) => {
                link = None;
            }
            Event::Start(Tag::Image { ref dest_url, .. }) => {
                if let Some(l) = &link {
                    shields.push(format!("[![]({})]({})", dest_url, l));
                } else {
                    shields.push(format!("![]({})", dest_url));
                }
            }
            Event::Start(Tag::Heading { .. }) => {
                break;
            }
            _ => {}
        }
    }
    Ok(shields)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple() -> Result<()> {
        let md = "[![alt1](img1)](link1)
![alt2](img2)
[![alt3](img2)](link3)

# github-user-repos-status

[![alt4](img2)](link3)

        ";
        let shields = shields_get(&md)?;
        assert_eq!(
            shields,
            ["[![](img1)](link1)", "![](img2)", "[![](img2)](link3)"]
        );
        Ok(())
    }
}
