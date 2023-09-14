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

use rayon::prelude::*;
use rayon::Scope;

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("data/example.zip");

    // Open the ZIP file for reading.
    let file = File::open(&path)?;

    let file_regex=get_file_regex();
    let data_regex=get_data_regex();


    rayon::scope( |s: &Scope| {
        for _thread in 0..100 {
            s.spawn( |_s| {
                process_zip(&file,&file_regex,&data_regex).unwrap();
            })
        }
    });

    Ok(())
}

fn process_zip(file: &File,file_regex: &regex::Regex, data_regex: &regex::Regex) -> Result<(),Box<dyn Error>> {
    let mut archive = ZipArchive::new(file)?;
    // Iterate through all the files in the ZIP archive.
    for n in 0..archive.len() {
        let mut file = archive.by_index(n)?;
        if file_regex.find(file.name()).unwrap().is_empty() {
            continue;
        }
        println!("File name: {}", file.name());
        let mut s :String=String::from("");
        file.read_to_string(&mut s)?;
        print!("{}",s);
        s.lines()
            .map(|line| data_regex.captures(line))
            .for_each(|x| println!("{:?}",x));
    }
    Ok(())
}

fn get_file_regex( ) -> regex::Regex {
    let file_regex=RegexBuilder::new(
        r#"(?<Text>[^*.txt$|*.json])"#)
        .case_insensitive(false)
        .build()
        .unwrap();
    file_regex
}

fn get_data_regex() -> regex::Regex {
    let data_regex = RegexBuilder::new( 
        r#"(?<Hello_column>Hello), (?<World_column>W.*)"#)
        .case_insensitive(true)
        .build().unwrap();
    data_regex
}

fn create_zip( path : &String) -> Result<(),Box<dyn Error>>
{
    fs::create_dir_all("data")?;

    {
    let file = File::create(&path)?;

    let mut zip = ZipWriter::new(file);

    zip.start_file("readme.txt", FileOptions::default())?;
    zip.write_all(b"Hello, World!\n")?;
    zip.finish()?;
    
    println!("Zip file created successfully!");
    }
    Ok(())
}
