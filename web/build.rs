//! Build: compile `content/posts/*.md` into a static `POSTS` array.
//! Only `*.md` regular files. Required: non-empty `title`, valid ISO `date`.
//! Validation lives in the shared `src/content.rs` module (included via `#[path]`).
//! Deterministic order: date desc, then slug asc. Empty set is valid.

#[path = "src/content.rs"]
mod content;

use std::{
    fs,
    path::{Path, PathBuf},
};

use pulldown_cmark::{Event, Options, Parser, html};

fn main() {
    let dir = PathBuf::from("../content/posts");
    println!("cargo:rerun-if-changed={}", dir.display());

    let mut posts = load_posts(&dir);
    // date desc, then slug asc for deterministic same-date order
    posts.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.2.cmp(&b.2)));

    let mut out = String::from("pub static POSTS: &[Post] = &[\n");
    for (code, _, _) in &posts {
        out.push_str(code);
        out.push_str(",\n");
    }
    out.push_str("];\n");

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR");
    fs::write(Path::new(&out_dir).join("content.rs"), out).expect("write content.rs");
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("{}: {err}", path.display()))
}

/// Returns (rust literal, sort-date, slug). Missing dir means no posts.
fn load_posts(dir: &Path) -> Vec<(String, String, String)> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Vec::new(),
        Err(err) => panic!("{}: {err}", dir.display()),
    };
    entries
        .map(|entry| entry.unwrap_or_else(|err| panic!("{}: {err}", dir.display())))
        .filter(|entry| {
            entry
                .file_type()
                .unwrap_or_else(|err| panic!("{}: {err}", entry.path().display()))
                .is_file()
                && entry.path().extension().is_some_and(|ext| ext == "md")
        })
        .map(|entry| render_post(&entry.path()))
        .collect()
}

fn render_post(path: &Path) -> (String, String, String) {
    println!("cargo:rerun-if-changed={}", path.display());
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("?");
    let raw = read(path);
    let slug = path
        .file_stem()
        .and_then(|s| s.to_str())
        .expect("post filename");

    content::validate_slug(slug, name).unwrap_or_else(|e| panic!("{e}"));

    let (meta, body) = split_front_matter(&raw, name);
    let html = markdown_to_html(body);
    let minutes = reading_minutes(&plain_text(body));
    let summary = meta
        .summary
        .unwrap_or_else(|| plain_excerpt(&plain_text(body), 120));
    let year = meta.date.get(..4).unwrap_or_default().to_string();
    let date_display = meta.date.replace('-', ".");
    let rss_date = content::iso_to_rfc822(&meta.date);
    let tags = meta
        .tags
        .iter()
        .map(|t| format!("{t:?}"))
        .collect::<Vec<_>>()
        .join(", ");

    let code = format!(
        "Post {{ slug: {slug:?}, title: {title:?}, date: {date:?}, date_display: {dd:?}, \
         year: {year:?}, summary: {summary:?}, tags: &[{tags}], minutes: {min}, \
         html: {html:?}, rss_date: {rss:?} }}",
        title = meta.title,
        date = meta.date,
        dd = date_display,
        year = year,
        summary = summary,
        tags = tags,
        min = minutes,
        html = html,
        rss = rss_date,
    );
    (code, meta.date, slug.to_string())
}

struct Meta {
    title: String,
    date: String,
    summary: Option<String>,
    tags: Vec<String>,
}

/// Documented simple `key: value` front matter. No YAML: no nesting, flow maps,
/// block scalars or lists. `tags` is a comma-separated value, optionally `[...]`.
fn split_front_matter<'a>(raw: &'a str, name: &str) -> (Meta, &'a str) {
    let rest = raw
        .strip_prefix("---\n")
        .unwrap_or_else(|| panic!("{name}: missing opening '---' front matter"));
    let (front, body) = rest
        .split_once("\n---\n")
        .unwrap_or_else(|| panic!("{name}: unterminated front matter"));

    let mut title = String::new();
    let mut date = String::new();
    let mut summary = None;
    let mut tags = Vec::new();

    for line in front.lines() {
        if line.starts_with(' ')
            || line.starts_with('\t')
            || line.starts_with('#')
            || line.starts_with('-')
            || line.contains(": ") && line.trim_start().starts_with('-')
        {
            panic!("{name}: YAML-style line not supported: {line}");
        }
        let (key, value) = line
            .split_once(':')
            .unwrap_or_else(|| panic!("{name}: malformed front matter line: {line}"));
        let key = key.trim();
        let value = value.trim().trim_matches(['"', '\'']).to_string();

        match key {
            "title" => title = value,
            "date" => date = value,
            "summary" => summary = Some(value),
            "tags" => {
                tags = value
                    .trim_matches(['[', ']'])
                    .split(',')
                    .map(|t| t.trim().trim_matches(['"', '\'']).to_string())
                    .filter(|t| !t.is_empty())
                    .collect();
            }
            "" => panic!("{name}: empty metadata key in line: {line}"),
            other => panic!("{name}: unknown metadata key '{other}'"),
        }
    }

    if title.trim().is_empty() {
        panic!("{name}: title must not be empty");
    }
    content::validate_date(&date, name).unwrap_or_else(|e| panic!("{e}"));

    (
        Meta {
            title,
            date,
            summary,
            tags,
        },
        body,
    )
}

fn markdown_to_html(markdown: &str) -> String {
    const OPTIONS: Options = Options::ENABLE_TABLES
        .union(Options::ENABLE_FOOTNOTES)
        .union(Options::ENABLE_STRIKETHROUGH)
        .union(Options::ENABLE_SMART_PUNCTUATION);
    let mut html = String::with_capacity(markdown.len() * 2);
    html::push_html(&mut html, Parser::new_ext(markdown, OPTIONS));
    html
}

/// Plain text from parsed Markdown events: no HTML tags, no raw HTML/script leakage.
fn plain_text(markdown: &str) -> String {
    const OPTIONS: Options = Options::ENABLE_TABLES
        .union(Options::ENABLE_FOOTNOTES)
        .union(Options::ENABLE_STRIKETHROUGH)
        .union(Options::ENABLE_SMART_PUNCTUATION);
    let mut text = String::new();
    for event in Parser::new_ext(markdown, OPTIONS) {
        match event {
            Event::Text(t) | Event::Code(t) | Event::InlineMath(t) | Event::DisplayMath(t) => {
                text.push_str(&t);
                text.push(' ');
            }
            Event::SoftBreak | Event::HardBreak => text.push(' '),
            _ => {}
        }
    }
    text
}

/// CJK counted per char, Western words count as 2 chars; 200 chars/min, ceil.
fn reading_minutes(text: &str) -> usize {
    let (mut cjk, mut words, mut in_word) = (0usize, 0usize, false);
    for ch in text.chars() {
        if ('\u{3400}'..='\u{9fff}').contains(&ch) {
            cjk += 1;
            in_word = false;
        } else if ch.is_alphanumeric() {
            if !in_word {
                words += 1;
                in_word = true;
            }
        } else {
            in_word = false;
        }
    }
    ((cjk + words * 2).max(1)).div_ceil(200)
}

fn plain_excerpt(text: &str, max_chars: usize) -> String {
    let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut excerpt: String = text.chars().take(max_chars).collect();
    if text.chars().count() > max_chars {
        excerpt.push('\u{2026}');
    }
    excerpt
}
