use std::{
    fs,
    path::{Path, PathBuf},
    collections::HashMap,
    time::SystemTime,
};
use ramhorns::{Template, Content};
use chrono::prelude::*;
use crate::route::Route;

pub struct ExtMap<'a> {
    map: HashMap<String, Template<'a>>
}

impl<'a> ExtMap<'a> {
    pub fn new(folder: PathBuf) -> ExtMap<'a> {
        let mut map = HashMap::new();
        let paths = fs::read_dir(folder)
            .expect("Could not read dump extensions dir");

        for raw_path in paths {
            let path = raw_path.unwrap().path().to_path_buf();
            let extension = path.file_stem()
                .expect("Could not get extension type")
                .to_os_string().into_string().unwrap();

            let template = build_template(path);
            map.insert(extension, template);
        }

        ExtMap { map }
    }

    /// Takes an extension, an environment, and some context,
    /// and renders it into some html,
    /// which it then inserts into the base template.
    pub fn render(&self, ext: &str, env: &Env, mut context: Context) -> String {
        let template = match self.map.get(ext) {
            Some(t) => t,
            None => match context.content {
                Some(_) => &env.text_template,
                None    => &env.binary_template,
            }
        };

        let inner = template.render(&context);
        context.content = Some(inner);
        env.base_template.render(&context)
    }
}

#[derive(Clone, Content, PartialEq, Eq)]
pub struct Metadata {
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
            created:  created.format("%F").to_string(),
            modified: modified.format("%F").to_string(),
            title:    route.title(),
            link:     format!("./{}", route.route.join("/")),
            raw_link: format!("./{}{}", route.route.join("/"), route.ext()),
            slug:     route.slug(),
            raw_slug: route.slug_with_ext(),
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
pub struct Context {
    pub metadata: Metadata,
    pub parent:   Option<Metadata>,
    pub children: Vec<Metadata>,
    /// If the context is valid text, this is it
    pub content:  Option<String>,
}

impl Context {
    pub fn new(
        metadata: Metadata,
        parent:   Option<Metadata>,
        children: Vec<Metadata>,
        raw_content: Vec<u8>
    ) -> Context {
        let content = match std::str::from_utf8(&raw_content) {
            Ok(v)  => Some(v.to_string()),
            Err(_) => None,
        };

        Context {
            metadata,
            parent,
            children,
            content,
        }
    }
}

pub fn build_template<'a>(path: PathBuf) -> Template<'a> {
    let contents = fs::read_to_string(path).expect("Could not read template");
    let template = Template::new(contents).expect("Could not build template");
    return template;
}

pub struct Env<'a> {
    pub base_template:   Template<'a>,
    pub index_template:  Template<'a>,
    pub text_template:   Template<'a>,
    pub binary_template: Template<'a>,
}

impl<'a> Env<'a> {
    pub fn base(path: PathBuf) -> Env<'a> {
        Env {
            base_template:   build_template(path.join("base.mustache")),
            index_template:  build_template(path.join("index.mustache")),
            text_template:   build_template(path.join("text.mustache")),
            binary_template: build_template(path.join("binary.mustache")),
        }
    }
}
