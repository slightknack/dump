use std::{
    fs,
    time::SystemTime,
};
use ramhorns::Content;
use chrono::prelude::*;
use pulldown_cmark::{Parser, Options, html};
use crate::route::Route;

#[derive(Debug, Clone, Content, PartialEq, Eq)]
pub struct Metadata {
    /// Time file was created in RFC 2822
    pub published: String,
    /// Time file was created as `Month Day, Year`
    pub created_human: String,
    /// Time file was last modified as `Month Day, Year`
    pub modified_human: String,
    /// Time file was created
    pub created: String,
    /// Time file was last modified
    pub modified: String,
    /// Title of the item
    pub title: String,
    /// Link to the item's page
    pub link: String,
    /// Link to the item's source
    pub raw_link: String,
    /// Last part of the link
    pub slug: String,
    /// Last part of the raw link
    pub raw_slug: String,
}

fn time_in_utc(t: SystemTime) -> DateTime<Utc> {
    if let Ok(dur) = t.duration_since(SystemTime::UNIX_EPOCH) {
        Utc.timestamp(dur.as_secs() as i64, dur.subsec_nanos())
    } else {
        panic!("Could not determine time")
    }
}

impl Metadata {
    pub fn from_route(route: &Route) -> Metadata {
        let fs_meta = fs::metadata(&route.path).expect("Could not read fs metadata");
        let created = time_in_utc(fs_meta.created().unwrap());
        let modified = time_in_utc(fs_meta.modified().unwrap());

        Metadata {
            published: created.format("%a, %d %b %Y %T UTC").to_string(),
            created_human: created.format("%A %B %e, %Y").to_string(),
            modified_human: modified.format("%A %B %e, %Y").to_string(),
            created:   created.format("%F").to_string(),
            modified:  modified.format("%F").to_string(),
            title:     route.title(),
            link:      route.route.join("/"),
            raw_link:  format!("{}{}", route.route.join("/"), route.ext()),
            slug:      route.slug(),
            raw_slug:  route.slug_with_ext(),
        }
    }
}

impl Ord for Metadata {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Works because created is YYYY-MM-DD
        self.created.cmp(&other.created)
    }
}

impl PartialOrd for Metadata {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Content)]
pub struct Markdown {
    content: String,
}

impl Markdown {
    pub fn new(raw: &str) -> Markdown {
        let parser = Parser::new_ext(&raw, Options::all());
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        return Markdown { content: html_output };
    }
}

#[derive(Content)]
pub struct Context {
    pub m: Metadata,
    /// TODO: vec of metadata?
    pub parent:   Option<Metadata>,
    pub children: Vec<Metadata>,
    /// If the context is valid text, this is it
    pub content:  Option<String>,
    /// If the content is valid markdown, this is it
    pub md: Option<Markdown>,
}

impl Context {
    pub fn new(
        metadata: Metadata,
        parent:   Option<Metadata>,
        mut children: Vec<Metadata>,
        raw_content: Vec<u8>
    ) -> Context {
        children.sort();
        let content = match std::str::from_utf8(&raw_content) {
            Ok(v)  => Some(v.to_string()),
            Err(_) => None,
        };

        Context {
            m: metadata,
            parent,
            children,
            md: content.clone().map(|i| Markdown::new(&i)),
            content,
        }
    }
}
