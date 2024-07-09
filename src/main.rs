use std::error::Error;
use pulldown_cmark::{html, Parser, Event, Tag, TagEnd, Options, CodeBlockKind};
use pulldown_cmark::utils::TextMergeStream;

enum BlockType {
    THEOREM,
}

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
                let block_title = block_title.split('=').last().unwrap_or("").trim();
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

fn main() {}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_global() {
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
</div>
<p>Some more text.</p>
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
</div>
"#;
            assert_eq!(expected_html, format_markdown(markdown))
        
}
    #[test]
    fn test_markdown() {
        let markdown = r#" Text before
```theorem title=nathan
Some text```
Text that shouldn't be in the code block
```theorem title=adrien
J'aurais pas mon permis d'apres Adrien
```
Une phrase"#;
        let html = format_markdown(markdown);
        println!("{}", html);
    }
}