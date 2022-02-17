use std::fs;
use walkdir;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author="Scott Sloane", version, about = "Copy all media from a directory to another", long_about = "Copies all media files from the source directory and it's subfolders into the output directory.")]
struct Args {
    
    /// Place all thumbnails in main output directory instead of nested Thumbs directory
    #[clap(long, short)]
    nothumb: bool,

    /// Comma Seperated file extentions that will be copied
    #[clap(long,short, default_value="jpg,png,gif,mp4,mov,webp,pdf,jpeg")]
    exts: String,

    /// Do not copy any files output to terminal instead
    #[clap(long, short)]
    dry: bool,

    /// Path to the root source directory
    #[clap()]
    source: String,

    /// Path to the output directory
    #[clap()]
    destination: String,
}

// Function to determine if a file is of a matching media type
fn is_media(file: &str, exts: &Vec<String>) -> bool {

    for ext in exts {
        let file_ext = format!(".{}", ext);
        let file_ext_cap = file_ext.to_uppercase();
        if file.ends_with(&file_ext) || file.ends_with(&file_ext_cap) { return true }
    }
    return false;
}

// Create the output directory if it does not already exist
fn ensure_dir(dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(dir)?;
    Ok(())
}

// Main function
fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Args::parse();
    let out_dir = &args.destination;
    let thumb_dir = format!("{}{}", &out_dir, "\\Thumbs");
    let in_dir = &args.source;

    let exts: Vec<String> = args.exts.split(",").map(|s| s.to_string()).collect();

    ensure_dir(&out_dir).expect("Could not create output directory");

    if !args.nothumb  {
        ensure_dir(&thumb_dir).expect("Could not create thumbnail directory");
    } 

    // Loop through all files in the input directory
    for entry in walkdir::WalkDir::new(&in_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok()) {
        let f_name = entry.file_name().to_string_lossy();
        let in_path = entry.path();

        if is_media(&f_name, &exts) { 
            let out_path = format!("{}\\{}", if f_name.contains("thumb") && !args.nothumb  {thumb_dir.as_str()} else {out_dir}, f_name);
            if args.dry {
                // if dry run just output info to the colsole
                println!("{}", out_path);
            }else {
                // copy the file to the output directory
                fs::copy(in_path, out_path)?;
            }
        }
    }

    Ok(())
}