use std::{
    env,
    path::PathBuf,
};

// mod render;
mod dump;
mod route;
mod render;

fn main() {
    let input_path = env::current_dir()
        .expect("Could not get current directory");
    let output_path = env::args_os().nth(1)
        .expect("Usage: <blank-output-path>");

    let base       =   dump::Env::base(input_path.join(".dump"));
    let extensions = dump::ExtMap::new(input_path.join(".dump").join("ext"));

    // start recursive rendering process with root page
    let base_root = route::Route::root(input_path);

    // render out to a tmp dir, then move?
    render::render(None, &base_root, &base, &extensions, PathBuf::from(output_path));
}
