use std::{
    time,
    env,
    path::PathBuf,
};

mod dump;
mod metadata;
mod route;
mod render;

fn main() {
    let start = time::Instant::now();
    let input_path = env::current_dir()
        .expect("Could not get current directory");
    let output_path = env::args_os().nth(1)
        .expect("Usage: <blank-output-path>");

    let base = dump::Env::base(input_path.join(".dump"));
    let mut extensions = dump::ExtMap::new(base, input_path.join(".dump").join("ext"));

    // start recursive rendering process with root page
    let base_root = route::Route::root(input_path);

    // render out to a tmp dir, then move?
    render::render(None, &base_root, &mut extensions, PathBuf::from(output_path));
    let taken = time::Instant::now().duration_since(start).as_millis();
    println!("Walked in {}s", taken as f64 / 1000.0);
}
