use regex::Regex;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/*
    Structures and Enums
*/

enum LineToken {
    Heading1(String),
    Heading2(String),
    Heading3(String),
    Author(String),
    Paragraph(String),
    List(ListItem),
    CodeBlock(String),
    /*
        later use prism.js to highlight code
    */
    Quote(String),
    HorizontalRule,
    Image(String, String, String),
    Table(String),
    Empty,
}

struct Comment {
    comment_token: String,
    replaceable: String,
    reference: String,
}

struct ListItem {
    text: String,
    level: usize,
}

#[derive(PartialEq, Eq)]
enum MultiLineState {
    List,
    CodeBlock,
    Table,
    None,
}

/*
    Utility Functions
*/

fn hash_string_to_integer(input: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}

fn modifiy_text_with_design(text: String, comments: &mut Vec<String>) -> String {
    // substitute all escape characters
    /*
        \#\*\-\[\]\(\)\{\}\!\`\>\|\<\>\~\$\?
    */
    let text = text.replace(r"\#", "\\+hash");
    let text = text.replace(r"\*", "\\+star");
    let text = text.replace(r"\-", "\\+dash");
    let text = text.replace(r"\[", "\\+leftsquare");
    let text = text.replace(r"\]", "\\+rightsquare");
    let text = text.replace(r"\(", "\\+leftparen");
    let text = text.replace(r"\)", "\\+rightparen");
    let text = text.replace(r"\{", "\\+leftcurly");
    let text = text.replace(r"\}", "\\+rightcurly");
    let text = text.replace(r"\!", "\\+exclamation");
    let text = text.replace(r"\`", "\\+backtick");
    let text = text.replace(r"\>", "\\+greater");
    let text = text.replace(r"\|", "\\+pipe");
    let text = text.replace(r"\<", "\\+lesser");
    let text = text.replace(r"\>", "\\+greater");
    let text = text.replace(r"\~", "\\+tilde");
    let text = text.replace(r"\$", "\\+dollar");
    let text = text.replace(r"\?", "\\+question");

    // substitute ***text*** with <b><i> text </i></b>
    let bold_italic_regex = Regex::new(r"\*\*\*(.*?)\*\*\*").unwrap();
    let text = bold_italic_regex
        .replace_all(&text, "<b><i>$1</i></b>")
        .to_string();

    // substitute **text** with <b> text </b>
    let bold_regex = Regex::new(r"\*\*(.*?)\*\*").unwrap();
    let text = bold_regex.replace_all(&text, "<b>$1</b>").to_string();

    // substitute *text* with <i> text </i>
    let italic_regex = Regex::new(r"\*(.*?)\*").unwrap();
    let text = italic_regex.replace_all(&text, "<i>$1</i>").to_string();

    // substitute ~~text~~ with <s> text </s>
    let strikethrough_regex = Regex::new(r"~~(.*?)~~").unwrap();
    let text = strikethrough_regex
        .replace_all(&text, "<s>$1</s>")
        .to_string();

    // substitute ~text~ with <u> text </u>
    let underline_regex = Regex::new(r"~(.*?)~").unwrap();
    let text = underline_regex.replace_all(&text, "<u>$1</u>").to_string();

    // substitute `text` with <code> text </code>
    let code_regex = Regex::new(r"`(.*?)`").unwrap();
    let text = code_regex
        .replace_all(&text, "<span class=\"monospace\">$1</span>")
        .to_string();

    // subsitute <red>text</red> with <span class="red-text"> text </span>
    // subsitute <teal>text</teal> with <span class="teal-text"> text </span>
    // subsitute <green>text</green> with <span class="green-text"> text </span>
    // subsitute <orange>text</orange> with <span class="orange-text"> text </span>
    // allowed color red, teal, green, orange
    let color_regex =
        Regex::new(r"<(red|teal|green|orange)>(.*?)</(red|teal|green|orange)>").unwrap();
    let text = color_regex
        .replace_all(&text, "<span class=\"$1-text\">$2</span>")
        .to_string();

    // allowed highlight colors: red, green, yellow, pink
    // subsitute <!red>text</!red> with <span class="red-highlight"> text </span>
    // subsitute <!green>text</!green> with <span class="green-highlight"> text </span>
    // subsitute <!yellow>text</!yellow> with <span class="yellow-highlight"> text </span>
    // subsitute <!pink>text</!pink> with <span class="pink-highlight"> text </span>
    let highlight_regex =
        Regex::new(r"<\!(red|green|yellow|pink)>(.*?)</\!(red|green|yellow|pink)>").unwrap();
    let text = highlight_regex
        .replace_all(&text, "<span class=\"$1-highlight\">$2</span>")
        .to_string();

    // substitute [text](link) with <a href="link"> text </a>
    let link_regex = Regex::new(r"\[(.*?)\]\((.*?)\)").unwrap();
    let text = link_regex
        .replace_all(&text, "<a href=\"$2\">$1</a>")
        .to_string();

    // substitute <spoiler>text</spoiler> with <span class="spoiler"> text </span>
    let spoiler_regex = Regex::new(r"<spoiler>(.*?)</spoiler>").unwrap();
    let text = spoiler_regex
        .replace_all(&text, "<span class=\"spoiler\">$1</span>")
        .to_string();

    // substitute $$text$$ with \[ text \]
    let block_equation_regex = Regex::new(r"\$\$(.*?)\$\$").unwrap();
    let text = block_equation_regex
        .replace_all(&text, "\\[$1\\]")
        .to_string();

    // substitue $text$ with \( text \)
    let inline_equation_regex = Regex::new(r"\$(.*?)\$").unwrap();
    let text = inline_equation_regex
        .replace_all(&text, "\\($1\\)")
        .to_string();

    // substitute (text)(This comment explains the text on side) with <span class="comment"> text </span>
    let comment_regex = Regex::new(r"\((.*?)\)\((.*?)\)").unwrap();
    let mut comments_found: Vec<Comment> = Vec::new();
    // capture all comments
    for comment in comment_regex.captures_iter(&text) {
        let comment_text = format!("({})({})", &comment[1], &comment[2]);
        let comment_id = hash_string_to_integer(&comment_text);
        comments_found.push(Comment {
            comment_token: comment_text,
            replaceable: format!(
                "<span class=\"comment\" data-target={}>{}</span>",
                comment_id, &comment[1]
            ),
            reference: format!(
                "<div id={} class=\"comment_explain hidden\">{}</div>",
                comment_id, &comment[2]
            ),
        });
    }
    // replace all comments
    let mut replaced_text = text.clone();
    for comment in comments_found {
        replaced_text = replaced_text.replace(&comment.comment_token, &comment.replaceable);
        comments.push(comment.reference);
    }
    let text = replaced_text;

    // substitute back all escape characters
    let text = text.replace("\\+hash", "#");
    let text = text.replace("\\+star", "*");
    let text = text.replace("\\+dash", "-");
    let text = text.replace("\\+leftsquare", "[");
    let text = text.replace("\\+rightsquare", "]");
    let text = text.replace("\\+leftparen", "(");
    let text = text.replace("\\+rightparen", ")");
    let text = text.replace("\\+leftcurly", "{");
    let text = text.replace("\\+rightcurly", "}");
    let text = text.replace("\\+exclamation", "!");
    let text = text.replace("\\+backtick", "`");
    let text = text.replace("\\+greater", ">");
    let text = text.replace("\\+pipe", "|");
    let text = text.replace("\\+lesser", "&lt;");
    let text = text.replace("\\+greater", "&gt;");
    let text = text.replace("\\+tilde", "~");
    let text = text.replace("\\+dollar", "$");
    let text = text.replace("\\+question", "?");

    text
}

/*
    Tokenizer Functions
*/

fn line_tokenizer(line: &str) -> LineToken {
    if line.is_empty() {
        LineToken::Empty
    } else if line.starts_with("# ") {
        LineToken::Heading1(line[2..].to_string())
    } else if line.starts_with("## ") {
        LineToken::Heading2(line[3..].to_string())
    } else if line.starts_with("### ") {
        LineToken::Heading3(line[4..].to_string())
    } else if line.starts_with("#### ") {
        LineToken::Heading3(line[5..].to_string())
    } else if line.starts_with("##### ") {
        LineToken::Heading3(line[6..].to_string())
    } else if line.starts_with("###### ") {
        LineToken::Heading3(line[7..].to_string())
    } else if line.starts_with("!# ") {
        LineToken::Author(line[3..].to_string())
    } else if line.starts_with("> ") {
        LineToken::Quote(line[2..].to_string())
    } else if line.starts_with("---") {
        LineToken::HorizontalRule
    } else if line.starts_with("![") {
        // first trim off the leading "!"
        let line = line[1..].to_string();
        let parts: Vec<&str> = line
            .split(|c| c == '[' || c == ']' || c == '(' || c == ')')
            .filter(|s| !s.is_empty())
            .collect();

        // Ensure we have at least the minimum required parts
        if parts.len() >= 3 {
            let alt = parts[0].to_string(); // First part is the alt text
            let height = parts[1].to_string(); // Second part is the height
            let source = parts[2].to_string(); // Last part is the image source

            LineToken::Image(source, alt, height)
        } else {
            LineToken::Image(
                "source".to_string(),
                "alt".to_string(),
                "height".to_string(),
            )
        }
    } else if line.starts_with("- ") {
        let mut indentation_count = 0;
        for c in line.chars() {
            if c == '-' {
                indentation_count += 1;
            }
        }
        indentation_count -= 1;
        LineToken::List(ListItem {
            text: line.trim().to_string().replace("- ", ""),
            level: indentation_count,
        })
    } else if line.starts_with("|") {
        // process table
        LineToken::Table(line.to_string())
    } else if line.starts_with("```") {
        LineToken::CodeBlock(line[3..].to_string())
    } else if line.starts_with("!") {
        LineToken::Empty
    } else {
        LineToken::Paragraph(line.to_string())
    }
}

fn token_to_html(token: LineToken, comments: &mut Vec<String>) -> String {
    // static storage: String = String::new();
    match token {
        LineToken::Heading1(text) => {
            format!("<h1>{}</h1>", text)
        }
        LineToken::Heading2(text) => {
            format!(
                "<div><h2>{}</h2><hr class=\"h-px bg-gray-200 border-0 dark:bg-gray-700\"></div>",
                text
            )
        }
        LineToken::Heading3(text) => {
            format!("<h3>{}</h3>", text)
        }
        LineToken::Author(text) => {
            format!("<div class=\"author\">{}</div>", text)
        }
        LineToken::Paragraph(text) => {
            let text = modifiy_text_with_design(text, comments);
            format!("<p>{}</p>", text)
        }
        LineToken::Quote(text) => {
            let text = modifiy_text_with_design(text, comments);
            format!("<div class=\"block_quote\"> 💬 {}</div>", text)
        }
        LineToken::HorizontalRule => "<hr>".to_string(),
        LineToken::Image(source, alt, height) => {
            format!(
                "<div class=\"flex flex-col justify-center items-center\"><img src=\"{}\" alt=\"{}\" style=\"height: {}; width: auto;\"><div class=\"text-sm\"><em>{}</em></div></div>",
                source, alt, height, alt
            )
        }
        LineToken::Empty => "".to_string(),
        LineToken::List(list_item) => {
            let symbol = match list_item.level % 3 {
                0 => "•",
                1 => "◦",
                2 => "▪",
                _ => "",
            };
            format!(
                "<div class=\"list_item pl-{} indent-{}\">{} {}</div>",
                list_item.level * 4,
                list_item.level,
                symbol,
                list_item.text
            )
        }
        LineToken::Table(text) => {
            let parsed_table_row = text
                .trim()
                .split("|")
                .filter(|x| !x.is_empty())
                .map(|x| x.trim())
                .map(|x| modifiy_text_with_design(x.to_string(), comments).to_string())
                .collect::<Vec<String>>();

            let mut table_html = String::new();
            for row in parsed_table_row {
                table_html.push_str(&format!("<div class=\"table_item\">{}</div>", row));
            }
            table_html
        }
        LineToken::CodeBlock(text) => {
            // return empty string for now
            text
        }
    }
}

fn markdown_to_html(markdown: String) -> String {
    let mut html = String::new();
    let mut is_code_block = false;

    // static storage of vectors of strings for comments
    let mut comments: Vec<String> = Vec::new();

    // switch states
    let mut state = MultiLineState::None;

    for line in markdown.lines() {
        // tokenize line
        let line_token = line_tokenizer(line);

        let new_state = match line_token {
            LineToken::List(_) => MultiLineState::List,
            LineToken::CodeBlock(_) => MultiLineState::CodeBlock,
            LineToken::Table(_) => MultiLineState::Table,
            _ => MultiLineState::None,
        };

        // if state swithces, add a newline
        if state != new_state {
            /*
                New State Fix
            */

            // if new state is List, add a <div class="list"> tag
            if new_state == MultiLineState::List {
                html.push_str("<div class=\"list\">\n");
            }

            // if new state is Table, add a <div class="table grid grid-cols-{} gap-4"> tag
            // count columns and add grid-cols-{} to the class
            if new_state == MultiLineState::Table {
                let count_columns = line.trim().split("|").filter(|x| !x.is_empty()).count();
                html.push_str(&format!(
                    "<div class=\"table grid grid-cols-{}\">\n",
                    count_columns
                ));
            }

            /*
                Previous State Fix
            */

            // if prev state is List, add a </div> tag
            if state == MultiLineState::List {
                html.push_str("</div>\n");
            }

            // if prev state is Table, add a </div> tag
            if state == MultiLineState::Table {
                html.push_str("</div>\n");
            }

            /*
                Code Block Fix
            */
            if new_state == MultiLineState::CodeBlock {
                if is_code_block {
                    html.push_str("</code></pre>\n");
                    is_code_block = false;
                    continue;
                } else {
                    let language = token_to_html(line_token, &mut comments);
                    html.push_str(
                        format!("<pre class=\"block_code {}\"><code>", language).as_str(),
                    );
                    is_code_block = true;
                    continue;
                }
            }
        }
        state = new_state;

        // convert token to html
        let line_html = token_to_html(line_token, &mut comments);

        // if we have a code block, we'll have a weird logic, sorry for that
        if is_code_block {
            // if code block, add the line as it is
            // just change < to &lt; and > to &gt;
            let line = line.replace("<", "&lt;");
            let line = line.replace(">", "&gt;");
            html.push_str(format!("{}\n", line).as_str());
            continue;
        }

        let line_html = format!("{}\n", line_html);

        // append to html
        html.push_str(&line_html);
    }

    // add comments
    for comment in comments {
        html.push_str(&format!("{}\n", comment));
    }

    html
}

/*
    Public Function
*/

pub fn start(markdown: String) -> String {
    markdown_to_html(markdown)
}   