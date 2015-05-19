use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let profile_dir = env::var("PROFILE").unwrap();
    let out_dir = Path::new("").join(manifest_dir).join("target").join(profile_dir);
    println!("Copying assets directory to output directory");
    // note that there are a number of downsides to this approach, the comments
    // below detail how to improve the portability of these commands.
    Command::new("cp").args(&["-r", "assets", out_dir.to_str().unwrap()])
                      .status().unwrap();
    println!("Done copying assets directory");
}