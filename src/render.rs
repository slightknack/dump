use std::{fs, path::PathBuf, io::Write, collections::HashSet};
use crate::{dump::{ExtMap, Env}, metadata::{Context, Metadata}, route::Route};

pub fn get_index_children(environment: &mut Env, route: &Route) -> (Option<Route>, Vec<Route>) {
    let mut index: Option<Route> = None;
    let mut children = vec![];
    let mut slugs    = HashSet::new();

    let paths = fs::read_dir(&route.path).unwrap();
    for raw_path in paths {
        let path = raw_path.unwrap().path().to_path_buf();
        let new_route = route.cd(path.file_name().unwrap().to_str().unwrap());

        if environment.ignores(&new_route.path) { continue; }
        if !slugs.insert(new_route.slug()) {
            eprintln!(
                "Multiple slugs of the name '{}'; first duplicate is: {}",
                new_route.slug(),
                new_route.path.display()
            );
            panic!();
        }

        children.push(new_route.clone());

        if path.is_file()
        && path.file_stem().unwrap() == "index" {
            index = match index {
                None => Some(new_route),
                Some(_) => panic!("Multiple indexes defined"),
            }
        }
    }

    return (index, children);
}

pub fn render(
    parent:      Option<Metadata>,
    route:       &Route,
    extensions:  &mut ExtMap,
    output_path: PathBuf,
) -> Metadata {
    println!("  {}", route.route.join("/"));
    fs::create_dir_all(output_path.clone())
        .expect("Could not create parent dir");

    let metadata = Metadata::from_route(&route);
    let (maybe_index, children) = get_index_children(&mut extensions.env, &route);
    let mut md_children = vec![];

    for child in children.iter() {
        let md_child = if child.path.is_file() {
            render_file(
                Some(metadata.clone()),
                child,
                vec![],
                extensions,
                output_path.clone(),
            )
        } else {
            render(
                Some(metadata.clone()),
                child,
                extensions,
                output_path.join(child.slug()),
            )
        };
        md_children.push(md_child);
    }

    // TODO: deal w/ maybe index.
    // todo!();

    return metadata;
}

pub fn render_file(
    parent:      Option<Metadata>,
    route:       &Route,
    children:    Vec<Metadata>,
    extensions:  &ExtMap,
    output_path: PathBuf,
) -> Metadata {
    println!("* {}", route.route.join("/"));

    let raw_content = fs::read(&route.path).unwrap();
    let mut raw_out = fs::File::create(output_path.join(format!("{}", route.slug_with_ext())))
        .expect("Could not create output raw file");
    raw_out.write_all(&raw_content)
        .expect("Could not write out raw");

    let metadata = Metadata::from_route(&route);
    let context = Context::new(
        metadata.clone(),
        parent,
        children,
        raw_content,
    );

    let rendered = extensions.render(&route.ext(), context);
    let mut render_out = fs::File::create(output_path.join(format!("{}.html", route.slug())))
        .expect("Could not create output render file");
    write!(render_out, "{}", rendered)
        .expect("Could not write out rendered");

    return metadata;
}
