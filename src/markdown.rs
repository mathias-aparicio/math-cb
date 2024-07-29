use std::{fs::File, io::Write, path::{self, Path}};

use pulldown_cmark::{html, Parser, Event, Tag, TagEnd, Options, CodeBlockKind};
use askama::{Error, Template}; // bring trait in scope

/// Convert a Markdown file to String, takes a path in any form
fn read_markdown<P: AsRef<Path>>(path: P) -> String {
    todo!();
}
enum BlockType {
    THEOREM,
}

/// format markdown to html
fn format_markdown(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    
    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    let mut in_code_block = false;
    let mut current_block_type = String::new();
    let mut current_block_title = String::new();
    let mut current_block_content = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(info))) => {
                in_code_block = true;
                let block_type = info.split_whitespace().next().unwrap_or("");
                let block_title = info.split_whitespace().skip(1).collect::<Vec<&str>>().join(" ");
                let block_title = block_title.split('=').last().unwrap_or("").trim().trim_matches('"');
                current_block_type = block_type.to_string();
                current_block_title = block_title.to_string();
            },
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;
                html_output.push_str(&format_block(&current_block_type, &current_block_title, &current_block_content));
                current_block_content.clear();
            },
            Event::Text(text) => {
                if in_code_block {
                    current_block_content.push_str(&text);
                } else {
                    html::push_html(&mut html_output, std::iter::once(Event::Text(text)));
                }
            },
            _ => {
                if !in_code_block {
                    html::push_html(&mut html_output, std::iter::once(event));
                }
            },
        }
    }

    html_output
}

fn to_block_type(block_type: &str) -> Result<BlockType, &'static str> {
    match block_type.to_lowercase().as_str() {
        "theorem" => Ok(BlockType::THEOREM),
        _ => Err("Bad, or not supported block")
    }
}

/// create the html for a block
fn format_block(block_type: &str, title: &str, content: &str) -> String {
    match to_block_type(block_type) {
        Ok(BlockType::THEOREM) => {
            format!(
r#"<div class="theorem">
    <div class="theorem-header">
        <div class="theorem-icon">
            <img src="https://via.placeholder.com/150" alt="Icon">
        </div>
        <div class="theorem-title">
            <p id="theorem-title">{}</p>
        </div>
    </div>
    <div class="theorem-content">
        <p>{}</p>
    </div>
</div>"#,
                title,
                content.trim()
            )
        }
        Err(_) => format!("<p>{}</p>", content) // Default to a simple paragraph if block type is not recognized
    }
}

    #[derive(Template)]
    #[template(path = "template.html")]
    struct PageTemplate<'a> {
        content: &'a str,
    }
/// Render the Askama template
pub fn render_page(markdown: &str) -> Result<String, Error>{
    // Instantiate the template
    let content = format_markdown(&markdown);
    let template = PageTemplate {content: &content};
    template.render()
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn test_format_markdown() {
        let markdown = r#"Some text *that is italic* and some **bold text**.
```theorem title="Théorème de Nathan" 
Soit $x$ un nombre réel. Alors $x^2 \geq 0$.
```
Some more text.
```theorem title="Théorème d'Adrien"
Si $x=0$ alors $2x=0$
        "#;
let expected_html = r#"<p>Some text <em>that is italic</em> and some <strong>bold text</strong>.</p>
<div class="theorem">
    <div class="theorem-header">
        <div class="theorem-icon">
            <img src="https://via.placeholder.com/150" alt="Icon">
        </div>
        <div class="theorem-title">
            <p id="theorem-title">Théorème de Nathan</p>
        </div>
    </div>
    <div class="theorem-content">
        <p>Soit $x$ un nombre réel. Alors $x^2 \geq 0$.</p>
    </div>
</div><p>Some more text.</p>
<div class="theorem"> 
    <div class="theorem-header">
        <div class="theorem-icon">
            <img src="https://via.placeholder.com/150" alt="Icon">
        </div>
        <div class="theorem-title">
            <p id="theorem-title">Théorème d'Adrien</p>
        </div>
    </div>
    <div class="theorem-content">
        <p>Si $x=0$ alors $2x=0$</p>
    </div>
</div>"#;
            assert_eq!(expected_html, format_markdown(markdown))
        
}
    

}