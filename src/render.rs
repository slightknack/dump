use std::{fs, path::PathBuf, io::Write, collections::HashSet};
use crate::{dump::{ExtMap, Env}, metadata::{Context, Metadata}, route::Route};

pub fn get_children(environment: &mut Env, route: &Route) -> (Option<Route>, Vec<Route>) {
    let mut index    = None;
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

        if new_route.slug() != "index" {
            children.push(new_route.clone());
        } else {
            index = Some(new_route.clone());
        }
    }

    return (index, children);
}

pub fn render(
    parent:      Option<Metadata>,
    route:       &Route,
    extensions:  &mut ExtMap,
    output_path: PathBuf,
    for_rss:     &mut Vec<Metadata>,
) -> Metadata {
    println!("|  {}", route.route.join("/"));
    fs::create_dir_all(output_path.clone())
        .expect("Could not create parent dir");

    let (index, children) = get_children(&mut extensions.env, &route);
    let mut md_children = vec![];

    let metadata = Metadata::from_route(&route);
    for_rss.push(metadata.clone());

    for child in children.iter() {
        let md_child = if child.path.is_file() {
            render_file(
                Some(metadata.clone()),
                child,
                extensions,
                output_path.clone(),
                for_rss,
            )
        } else {
            render(
                Some(metadata.clone()),
                child,
                extensions,
                output_path.join(child.slug()),
                for_rss,
            )
        };
        md_children.push(md_child);
    }

    return render_index(
        parent,
        metadata,
        index,
        md_children,
        extensions,
        output_path,
    );
}

pub fn render_index(
    parent:      Option<Metadata>,
    route:       Metadata,
    maybe_index: Option<Route>,
    children:    Vec<Metadata>,
    extensions:  &mut ExtMap,
    output_path: PathBuf,
) -> Metadata {
    let (raw_content, ext) = if let Some(index) = maybe_index {
        let raw = fs::read(&index.path).unwrap();
        // do not write raw index
        (raw, index.ext())
    } else {
        (vec![], "none".to_string())
    };

    let context = Context::new(
        route.clone(),
        parent.clone(),
        children.clone(),
        raw_content,
    );

    let rendered = extensions.render(&ext, context, true);
    let mut render_out = fs::File::create(output_path.join("index.html"))
        .expect("Could not create output render file");
    write!(render_out, "{}", rendered)
        .expect("Could not write out rendered");

    return route;
}

pub fn render_file(
    parent:      Option<Metadata>,
    route:       &Route,
    extensions:  &ExtMap,
    output_path: PathBuf,
    for_rss:     &mut Vec<Metadata>,
) -> Metadata {
    println!("|> {}", route.route.join("/"));

    let raw_content = fs::read(&route.path).unwrap();
    let mut raw_out = fs::File::create(output_path.join(format!("{}", route.slug_with_ext())))
        .expect("Could not create output raw file");
    raw_out.write_all(&raw_content)
        .expect("Could not write out raw");

    let metadata = Metadata::from_route(&route);
    for_rss.push(metadata.clone());
    let context = Context::new(
        metadata.clone(),
        parent,
        vec![],
        raw_content,
    );

    let actual_out = output_path.join(route.slug());
    fs::create_dir_all(actual_out.clone())
        .expect("Could not create parent dir");

    let rendered = extensions.render(&route.ext(), context, false);
    let mut render_out = fs::File::create(actual_out.join("index.html"))
        .expect("Could not create output render file");
    write!(render_out, "{}", rendered)
        .expect("Could not write out rendered");

    return metadata;
}
