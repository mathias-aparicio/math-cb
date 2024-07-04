use pulldown_cmark::{Parser, Event, Tag, Options, CodeBlockKind};
use pulldown_cmark::utils::{TextMergeStream};
fn format_markdown(markdown: &str) -> String {
    let options = Options::all();
    let parser = Parser::new_ext(markdown,options);
    let iterator = TextMergeStream::new(parser);
    let mut html = String::new();
    
    for event in iterator {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(info))) => {
                println!("info : {}", info);
                let block_type = info.split_whitespace().next().unwrap_or("");
                let block_title = info.split_whitespace().skip(1).collect::<Vec<&str>>().join(" ");
                println!("block_type : {}, block_title : {}", block_type, block_title)
            },
            _ => {}
        }
    }

    html
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global() {
        let markdown = r#"Some text *that is italic* and some **bold text**.
```Theoreme title="Théorème de Nathan" 
Soit $x$ un nombre réel. Alors $x^2 \geq 0$.
```
Some more text.
```Theoreme title="Théorème d'Adrien"
Si $x=0$ alors $2x=0$
        "#;
        let expected_html = r#"<p>Some text <em>that is italic</em> and some <strong>bold text</strong>.</p>
        <div class="theorem">
            <div class="theorem-header">
                <div class="theorem-icon">
                    <img src="https://via.placeholder.com/130" alt="Icon">
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
                    <img src="https://via.placeholder.com/130" alt="Icon">
                </div>
                <div class="theorem-title">
                    <p id="theorem-title">Théorème d'Adrien</p>
                </div>
            </div>   
            <div class="theorem-content">
                <p>Si $x=0$ alors $2x=0$</p>
            </div>
        "#;
}
    #[test]
    fn test_markdown() {
        let markdown = r#"```theorem title=nathan
Some text"#;
        let html = format_markdown(markdown);
    }
}