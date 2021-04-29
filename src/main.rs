use std::{
    time,
    env,
    fs,
    path::{PathBuf, Path},
};

mod dump;
mod metadata;
mod route;
mod render;

fn main() {
    let start = time::Instant::now();

    // get input and validate
    let input_path = env::current_dir()
        .expect("Could not get current directory");
    let output_path = env::args_os().nth(1)
        .expect("Usage: <blank-output-path>");
    let overwrite = match env::args_os().nth(2) {
        Some(flag) => flag == "--force",
        None       => false,
    };

    if !overwrite {
        if Path::new(&output_path).exists() {
            panic!("Output path already exists. Remove it and try again");
        }
    } else {
        fs::remove_dir_all(&output_path)
            .expect("Could not remove out dir");
    }


    // build the walking context from .dump
    let base = dump::Env::base(input_path.join(".dump"));
    let mut extensions = dump::ExtMap::new(base, input_path.join(".dump").join("ext"));

    // start recursive rendering process with root page
    let base_root = route::Route::root(input_path);

    // render out to a tmp dir, then move?
    render::render(None, &base_root, &mut extensions, PathBuf::from(output_path));

    // time it
    let taken = time::Instant::now().duration_since(start).as_millis();
    println!("Walked in {}s", taken as f64 / 1000.0);
}
