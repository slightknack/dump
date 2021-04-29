use std::path::{Path, PathBuf};
use deunicode::deunicode;

#[derive(Clone)]
pub struct Route {
    pub path:  PathBuf,
    pub route: Vec<String>,
}

impl Route {
    pub fn root(path: PathBuf) -> Route {
        Route { path, route: vec![] }
    }

    pub fn title(&self) -> String {
        match self.path.file_stem() {
            Some(s) => s.to_str().unwrap().to_string(),
            None => "".to_string(),
        }
    }

    pub fn ext(&self) -> String {
        let end = match self.path.extension() {
            Some(e) => e.to_str().unwrap().to_string(),
            None => return "".to_string(),
        };
        format!(".{}", end)
    }

    pub fn slug(&self) -> String {
        let name = self.title();
        deunicode(&name)
            .to_lowercase()
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { ' ' })
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join("-")
    }

    pub fn slug_with_ext(&self) -> String {
        let ext = self.ext();
        let slug = self.slug();
        return format!("{}{}", slug, ext);
    }

    pub fn cd(&self, item: &str) -> Route {
        let mut new = self.clone();
        new.path = self.path.join(item);
        new.route.push(new.slug());
        return new;
    }
}
