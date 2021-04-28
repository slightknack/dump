use std::{fs, path::PathBuf, io::Write};
use crate::{dump::{ExtMap, Env, Context, Metadata}, route::Route};

pub fn get_index_children(route: &Route) -> (Option<Route>, Vec<Route>) {
    let mut index: Option<Route> = None;
    let mut children = vec![];

    let paths = fs::read_dir(&route.path).unwrap();
    for raw_path in paths {
        let path = raw_path.unwrap().path().to_path_buf();
        let new_route = route.cd(path.file_name().unwrap().to_str().unwrap());
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
    environment: &Env,
    extensions:  &ExtMap,
    output_path: PathBuf,
) -> Metadata {
    println!("  {}", route.route.join("/"));
    fs::create_dir_all(output_path.clone())
        .expect("Could not create parent dir");

    let metadata = Metadata::from_route(&route);
    let (maybe_index, children) = get_index_children(&route);
    let mut md_children = vec![];

    for child in children.iter() {
        let md_child = if child.path.is_file() {
            render_file(
                Some(metadata.clone()),
                child,
                vec![],
                environment,
                extensions,
                output_path.clone(),
            )
        } else {
            render(
                Some(metadata.clone()),
                child,
                environment,
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
    environment: &Env,
    extensions:  &ExtMap,
    output_path: PathBuf,
) -> Metadata {
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

    let rendered = extensions.render(&route.ext(), environment, context);
    let mut render_out = fs::File::create(output_path.join(format!("{}.html", route.slug())))
        .expect("Could not create output render file");
    write!(render_out, "{}", rendered)
        .expect("Could not write out rendered");

    return metadata;
}
