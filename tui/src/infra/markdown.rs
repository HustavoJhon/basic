//! Extracción de metadatos Markdown para el índice.
//!
//! El renderizado visual vive en la capa `ui`; aquí solo se obtiene lo que
//! necesita SQLite y la búsqueda. Markdown sigue siendo la fuente de verdad.

use std::collections::BTreeSet;
use std::path::Path;

use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

/// Enlace encontrado en un documento.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkRef {
    pub target: String,
    pub text: Option<String>,
    pub kind: &'static str,
}

/// Metadatos derivados de un archivo Markdown.
#[derive(Debug, Clone)]
pub struct ParsedDocument {
    pub title: String,
    pub body: String,
    pub excerpt: String,
    pub plain_text: String,
    pub tags: Vec<String>,
    pub links: Vec<LinkRef>,
    pub word_count: usize,
    pub is_stub: bool,
    pub has_code: bool,
}

/// Parsea un documento completo y aplica los fallbacks aprobados en F0.
pub fn parse_document(raw: &str, relative_path: &str) -> ParsedDocument {
    let (frontmatter, body) = split_frontmatter(raw);
    let mut options = Options::all();
    options.remove(Options::ENABLE_SMART_PUNCTUATION);

    let mut title = None;
    let mut heading_text = String::new();
    let mut in_heading = false;
    let mut paragraph = String::new();
    let mut first_paragraph = None;
    let mut plain_text = String::new();
    let mut excerpt = String::new();
    let mut tags = BTreeSet::new();
    let mut links = Vec::new();
    let mut link_text = String::new();
    let mut in_link = false;
    let mut in_code_block = false;
    let mut has_code = false;

    for event in Parser::new_ext(body, options) {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                in_heading = true;
                heading_text.clear();
                if level == pulldown_cmark::HeadingLevel::H1 && title.is_none() {
                    // El título se fija al cerrar el heading.
                }
            }
            Event::End(TagEnd::Heading(level)) => {
                in_heading = false;
                let text = normalize_whitespace(&heading_text);
                if level == pulldown_cmark::HeadingLevel::H1 && title.is_none() && !text.is_empty()
                {
                    title = Some(text.clone());
                }
                if first_paragraph.is_none() && !text.is_empty() {
                    // Un documento que solo tiene headings sigue siendo útil.
                }
            }
            Event::Start(Tag::Paragraph) => {
                paragraph.clear();
            }
            Event::End(TagEnd::Paragraph) => {
                let text = normalize_whitespace(&paragraph);
                if first_paragraph.is_none() && !text.is_empty() {
                    first_paragraph = Some(text.clone());
                }
                paragraph.push(' ');
            }
            Event::Start(Tag::CodeBlock(_)) => {
                in_code_block = true;
                has_code = true;
            }
            Event::End(TagEnd::CodeBlock) => in_code_block = false,
            Event::Start(Tag::Link { dest_url, .. }) => {
                in_link = true;
                link_text.clear();
                let kind = link_kind(&dest_url);
                if kind != "external" || !dest_url.is_empty() {
                    links.push(LinkRef {
                        target: dest_url.to_string(),
                        text: None,
                        kind,
                    });
                }
            }
            Event::End(TagEnd::Link) => {
                in_link = false;
                if let Some(link) = links.last_mut() {
                    let text = normalize_whitespace(&link_text);
                    if !text.is_empty() {
                        link.text = Some(text);
                    }
                }
            }
            Event::Start(Tag::Image { dest_url, .. }) => {
                links.push(LinkRef {
                    target: dest_url.to_string(),
                    text: Some("imagen".to_string()),
                    kind: "image",
                });
            }
            Event::Text(text) => {
                if in_heading {
                    heading_text.push_str(&text);
                } else if in_link {
                    link_text.push_str(&text);
                } else if !in_code_block {
                    paragraph.push_str(&text);
                    plain_text.push_str(&text);
                    plain_text.push(' ');
                }
                if !in_heading && !in_code_block {
                    collect_inline_tags(&text, &mut tags);
                }
            }
            Event::Code(code) => {
                if !in_heading {
                    plain_text.push_str(&code);
                    plain_text.push(' ');
                }
            }
            Event::SoftBreak | Event::HardBreak if !in_code_block => {
                paragraph.push(' ');
                plain_text.push(' ');
            }
            _ => {}
        }
    }

    // Enlaces wiki, que CommonMark no reconoce.
    collect_wiki_links(body, &mut links);

    for tag in split_frontmatter_tags(&frontmatter) {
        tags.insert(tag);
    }

    let plain_text = normalize_whitespace(&plain_text);
    let word_count = plain_text.split_whitespace().count();
    if let Some(paragraph) = first_paragraph {
        excerpt = truncate(&paragraph, 180);
    }
    if excerpt.is_empty() {
        excerpt = truncate(&plain_text, 180);
    }

    ParsedDocument {
        title: title
            .filter(|title| !title.is_empty())
            .unwrap_or_else(|| fallback_title(relative_path)),
        body: body.to_string(),
        excerpt,
        is_stub: plain_text.is_empty(),
        plain_text,
        tags: tags.into_iter().collect(),
        links,
        word_count,
        has_code,
    }
}

/// Título de respaldo: nombre del archivo o carpeta para readmes.
fn fallback_title(relative_path: &str) -> String {
    let path = Path::new(relative_path);
    let stem = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("sin título");
    if stem.eq_ignore_ascii_case("readme") || stem.eq_ignore_ascii_case("index") {
        path.parent()
            .and_then(|parent| parent.file_name())
            .and_then(|name| name.to_str())
            .filter(|name| !name.is_empty())
            .map(prettify)
            .unwrap_or_else(|| prettify(stem))
    } else {
        prettify(stem)
    }
}

fn prettify(value: &str) -> String {
    let mut chars = value.replace(['_', '-'], " ").chars().collect::<Vec<_>>();
    if let Some(first) = chars.first_mut() {
        first.make_ascii_uppercase();
    }
    chars.into_iter().collect()
}

fn link_kind(target: &str) -> &'static str {
    if target.starts_with("http://")
        || target.starts_with("https://")
        || target.starts_with("mailto:")
    {
        "external"
    } else {
        "markdown"
    }
}

fn collect_wiki_links(body: &str, links: &mut Vec<LinkRef>) {
    let mut rest = body;
    while let Some(start) = rest.find("[[") {
        rest = &rest[start + 2..];
        let Some(end) = rest.find("]]") else {
            break;
        };
        let inner = &rest[..end];
        rest = &rest[end + 2..];
        let (target, text) = inner
            .split_once('|')
            .map(|(target, text)| (target, Some(text.to_string())))
            .unwrap_or((inner, None));
        let target = target.trim();
        if target.is_empty() {
            continue;
        }
        links.push(LinkRef {
            target: target.to_string(),
            text,
            kind: "wiki",
        });
    }
}

fn collect_inline_tags(text: &str, tags: &mut BTreeSet<String>) {
    for word in text.split_whitespace() {
        let candidate = word.trim_matches(|c: char| {
            !c.is_alphanumeric() && c != '#' && c != '-' && c != '_' && c != '/'
        });
        if let Some(tag) = candidate.strip_prefix('#') {
            let tag = tag.trim_matches(['.', ',', ';', ':', '!', '?', ')', ']']);
            if tag.len() >= 2 && tag.chars().any(|c| c.is_alphabetic()) {
                tags.insert(tag.to_lowercase());
            }
        }
    }
}

fn split_frontmatter(raw: &str) -> (String, &str) {
    let normalized = raw.strip_prefix('\u{feff}').unwrap_or(raw);
    if !normalized.starts_with("---") {
        return (String::new(), normalized);
    }
    let Some(end) = normalized[3..].find("\n---") else {
        return (String::new(), normalized);
    };
    let frontmatter = normalized[3..3 + end].trim().to_string();
    let body_start = 3 + end + 4;
    let body = normalized.get(body_start..).unwrap_or("");
    (frontmatter, body.trim_start_matches('\n'))
}

fn split_frontmatter_tags(frontmatter: &str) -> Vec<String> {
    let mut tags = Vec::new();
    let mut collecting = false;
    for line in frontmatter.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("tags:") {
            collecting = true;
            let inline = trimmed.trim_start_matches("tags:").trim();
            parse_tag_list(inline, &mut tags);
            if inline.starts_with('[') {
                collecting = false;
            }
        } else if collecting && trimmed.starts_with('-') {
            parse_tag_list(trimmed.trim_start_matches('-').trim(), &mut tags);
        } else if collecting && !trimmed.is_empty() && !trimmed.starts_with('#') {
            collecting = false;
        }
    }
    tags
}

fn parse_tag_list(value: &str, tags: &mut Vec<String>) {
    if value.is_empty() {
        return;
    }
    let value = value.trim_matches(['[', ']']);
    for tag in value.split(',') {
        let tag = tag.trim().trim_matches(['"', '\'']);
        if !tag.is_empty() {
            tags.push(tag.to_lowercase());
        }
    }
}

fn normalize_whitespace(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn truncate(value: &str, max_chars: usize) -> String {
    if value.chars().count() <= max_chars {
        return value.to_string();
    }
    let mut truncated: String = value.chars().take(max_chars.saturating_sub(1)).collect();
    truncated.push('…');
    truncated
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_title_excerpt_tags_and_links() {
        let parsed = parse_document(
            "# Ownership en Rust\n\nEl *préstamo* de memoria. #rust #ownership\n\n[Vim](../linux/vim.md)\n",
            "learning/rust/ownership.md",
        );
        assert_eq!(parsed.title, "Ownership en Rust");
        assert!(parsed.excerpt.contains("préstamo"));
        assert!(parsed.tags.contains(&"rust".to_string()));
        assert!(parsed.tags.contains(&"ownership".to_string()));
        assert_eq!(parsed.links[0].target, "../linux/vim.md");
        assert!(!parsed.is_stub);
    }

    #[test]
    fn falls_back_to_parent_for_readme() {
        let parsed = parse_document("# no importa\n\ncontenido", "desarrollo/php/README.md");
        assert_eq!(parsed.title, "no importa");
        let parsed = parse_document("", "desarrollo/php/readme.md");
        assert_eq!(parsed.title, "Php");
        assert!(parsed.is_stub);
    }

    #[test]
    fn parses_frontmatter_tags_and_wiki_links() {
        let parsed = parse_document(
            "---\ntags: [rust, memoria]\n---\n\n# Nota\n\n[[otra-nota|Otra nota]]\n",
            "notas/nota.md",
        );
        assert!(parsed.tags.contains(&"rust".to_string()));
        assert!(parsed.tags.contains(&"memoria".to_string()));
        assert_eq!(parsed.links[0].kind, "wiki");
        assert_eq!(parsed.links[0].text.as_deref(), Some("Otra nota"));
    }

    #[test]
    fn detects_empty_documents() {
        let parsed = parse_document("---\ntags: [vacia]\n---\n\n", "vacia.md");
        assert!(parsed.is_stub);
        assert_eq!(parsed.word_count, 0);
    }

    #[test]
    fn does_not_count_code_as_text() {
        let parsed = parse_document("# Nota\n\n```rust\nfn main() {}\n```\n", "nota.md");
        assert!(parsed.has_code);
        assert_eq!(parsed.plain_text, "");
        assert!(parsed.is_stub);
    }
}
