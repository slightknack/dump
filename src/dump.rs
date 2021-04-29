use std::{
    fs,
    path::{Path, PathBuf},
    collections::HashMap,
};
use ramhorns::Template;
use gitignored::Gitignore;
use crate::{dump_rss::RssConfig, metadata::Context};

pub struct ExtMap<'a> {
    pub env: Env<'a>,
    map: HashMap<String, Template<'a>>,
}

impl<'a> ExtMap<'a> {
    pub fn new(env: Env<'a>, folder: PathBuf) -> ExtMap<'a> {
        let mut map = HashMap::new();
        let paths = fs::read_dir(folder)
            .expect("Could not read dump extensions dir");

        for raw_path in paths {
            let path = raw_path.unwrap().path().to_path_buf();
            let extension = path.file_stem()
                .expect("Could not get extension type")
                .to_os_string().into_string().unwrap();

            let template = build_template(path);
            map.insert(format!(".{}", extension), template);
        }

        ExtMap { env, map }
    }

    /// Takes an extension, an environment, and some context,
    /// and renders it into some html,
    /// which it then inserts into the base template.
    pub fn render(&self, ext: &str, mut context: Context) -> String {
        let template = match self.map.get(ext) {
            Some(t) => t,
            None => match context.content {
                Some(_) => &self.env.text_template,
                None    => &self.env.binary_template,
            }
        };

        let inner = template.render(&context);
        context.content = Some(inner);
        self.env.base_template.render(&context)
    }
}

pub fn build_template<'a>(path: PathBuf) -> Template<'a> {
    let contents = fs::read_to_string(path).expect("Could not read template");
    let template = Template::new(contents).expect("Could not build template");
    return template;
}

pub struct Env<'a> {
    pub dump_rss:     Option<RssConfig>,
    pub dump_ignore:     Option<(Gitignore<PathBuf>, Vec<String>)>,
    pub base_template:   Template<'a>,
    pub index_template:  Template<'a>,
    pub text_template:   Template<'a>,
    pub binary_template: Template<'a>,
}

impl<'a> Env<'a> {
    pub fn load_ignore_cwd(file: PathBuf) -> Option<(Gitignore<PathBuf>, Vec<String>)> {
        let lines = fs::read_to_string(file).ok()?
            .lines()
            .map(|l| l.split("#").nth(0).unwrap().trim().to_string())
            .filter(|l| l != "")
            .collect();
        let ignore = Gitignore::default();
        return Some((ignore, lines));
    }

    pub fn ignores(&mut self, file: &Path) -> bool {
        if let Some((ref mut ignore, ref lines)) = self.dump_ignore {
            ignore.ignores(&lines.iter().map(|s| s as &str).collect::<Vec<&str>>(), file)
        } else {
            false
        }
    }

    pub fn base(path: PathBuf) -> Env<'a> {
        Env {
            dump_rss:        RssConfig::new(&path.join("dump_rss.toml")),
            dump_ignore:     Env::load_ignore_cwd(path.join(".dumpignore")),
            base_template:   build_template(path.join("base.mustache")),
            index_template:  build_template(path.join("index.mustache")),
            text_template:   build_template(path.join("text.mustache")),
            binary_template: build_template(path.join("binary.mustache")),
        }
    }
}
