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
mod dump_rss;

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

    let exists = Path::new(&output_path).exists();
    if overwrite {
        if exists {
            let _ = fs::remove_dir_all(&output_path);
        }
    } else {
        if exists {
            panic!("Output path already exists. Remove it and try again");
        }
    }

    println!("Dumping...");

    // build the walking context from .dump
    let base = dump::Env::base(input_path.join(".dump"));
    let mut extensions = dump::ExtMap::new(base, input_path.join(".dump").join("ext"));

    // start recursive rendering process with root page
    let base_root = route::Route::root(input_path);
    let mut for_rss = vec![];

    // render out to a tmp dir, then move?
    render::render(None, &base_root, &mut extensions, PathBuf::from(output_path.clone()), &mut for_rss);

    // do the RSS stuff
    if let Some(rss) = extensions.env.dump_rss {
        println!("Generating RSS feed...");
        rss.write_feed(Path::new(&output_path), for_rss);
    } else {
        println!("Not generating RSS feed.")
    }

    // time it
    let taken = time::Instant::now().duration_since(start).as_millis();
    println!("Dumped in {}s!", taken as f64 / 1000.0);
}
