use std::error::Error;

use std::io::Read;
use std::{path::Path, fs::File};

use zip::ZipArchive;
use zip::write::FileOptions;
use zip::ZipWriter;
use std::io::Write;
use std::collections::HashSet;
use std::fs;


use regex::RegexBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("data/example.zip");
    fs::create_dir_all("data")?;

    {
    let file = File::create(&path)?;

    let mut zip = ZipWriter::new(file);

    zip.start_file("readme.txt", FileOptions::default())?;
    zip.write_all(b"Hello, World!\n")?;
    zip.finish()?;
    
    println!("Zip file created successfully!");
    }

    // Open the ZIP file for reading.
    let file = File::open(&path)?;
    let mut archive = ZipArchive::new(file)?;

    let set = RegexBuilder::new(
        r#"(?<Hello_column>Hello), (?<World_column>W.*)"#
    ).case_insensitive(true)
        .build().unwrap();
    // Iterate through all the files in the ZIP archive.
    for n in 0..archive.len() {
        let mut file = archive.by_index(n)?;
        println!("File name: {}", file.name());
        let mut s :String=String::from("");
        file.read_to_string(&mut s)?;
        print!("{}",s);
        s.lines()
            .map(|line| set.captures(line))
            .for_each(|x| println!("{:?}",x));
    }

    Ok(())
}
