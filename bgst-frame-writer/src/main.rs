use anyhow::{Result, bail};
use clap::Parser;
use indicatif::{ProgressBar, ParallelProgressIterator};
use rayon::prelude::*;
use std::fs;

#[derive(Parser, Debug)]
struct Args {
    /// The name of the folder to read images from.
    folder_name: String,
    /// The name of the output file
    #[arg(default_value = "frames.bin")]
    file_name: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if !fs::exists(&args.folder_name)? {
        bail!("Folder '{}' not found", &args.folder_name);
    }

    let entries: Vec<_> = fs::read_dir(args.folder_name)?
        .filter_map(|res| res.ok())
        .collect();

    let pb = ProgressBar::new(entries.len() as u64);

    println!("Encoding images...");

    let out: Vec<u8> = entries
        .par_iter()
        .progress_with(pb)
        .filter_map(|entry| {
            let path = entry.path();
            let data = fs::read(path).ok()?;
            let img = image::load_from_memory(&data).ok()?;
            let rgba = img.into_rgba8().into_raw();

            Some(gctex::encode(gctex::TextureFormat::CMPR, &rgba, 512, 512))
        })
        .flatten()
        .collect();

    fs::write(args.file_name, out)?;

    println!("Done!");

    Ok(())
}
