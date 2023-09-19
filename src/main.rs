use std::error::Error;

use std::io::Read;
use std::{path::Path, fs::File};

use zip::ZipArchive;
use zip::write::FileOptions;
use zip::ZipWriter;
use std::io::Write;
use std::fs;

use regex::RegexBuilder;

use rayon::Scope;
use std::sync::{Mutex,Arc};

const FILE_LINES : i32 = 10_000;
const THREADS :i32 = 15;
const INPUT_FILES_PER_THREAD : i32 = 10;
const TOTAL_INPUT_FILES : i32 = THREADS*INPUT_FILES_PER_THREAD;
const DATA_DIR : &str = "data";
const OUTPUT_NAME: &str = "output.csv";


pub fn main() -> Result<(), Box<dyn Error>> {
    //create_files(TOTAL_INPUT_FILES)?;
    extract_files(THREADS,INPUT_FILES_PER_THREAD)
}


pub fn create_files(total_input_files: i32 ) -> Result<(), Box<dyn Error>> {
    // create as many files as threads to avoid collision 
    for thread in 0..total_input_files {
        let path_str=format!("{}/example_{:06}.zip",DATA_DIR,thread);
        create_zip(&path_str)?;
    }
    Ok(())
}
pub fn extract_files(threads: i32, input_files_per_thread: i32) -> Result<(), Box<dyn Error>> {

    //create threads to unzip the same file contents for illustrative purpose

    let output_file_path : String = format!("{}/{}",DATA_DIR,OUTPUT_NAME);
    let file = File::create(output_file_path)?;
    let out_file_mutex = Arc::new(Mutex::new(file));


    //let file_regex=Arc::new(&file_regex);
    //let data_regex=Arc::new(&data_regex);

    rayon::scope( |s: &Scope| {



        for thread in 0..threads {
            let start_file_index=input_files_per_thread*thread;

            let out_file_mutex=Arc::clone(&out_file_mutex);

            let file_regex=get_file_regex();
            let data_regex=get_data_regex();

            s.spawn(  move |_s| {
                let _process_results: Vec<_>=(start_file_index..start_file_index+INPUT_FILES_PER_THREAD)
                    .map( |file_index| format!("{}/example_{:06}.zip",DATA_DIR,file_index))
                    .map( |path_str| {
                        let path = Path::new(&path_str);
                        // Open the ZIP file for reading.
                        let file = File::open(&path).unwrap();
                        process_zip(&file,&file_regex,&data_regex,&out_file_mutex,&path_str)
                    }
                    ).collect();
            })
        }
    });

    Ok(())
}


fn process_zip(file: &File,file_regex: &regex::Regex, data_regex: &regex::Regex, output_file : &Arc::<Mutex<File>>,file_name: &String) -> Result<(),Box<dyn Error>> {
    let mut archive = ZipArchive::new(file)?;
    // Iterate through all the files in the ZIP archive.
    for n in 0..archive.len() {
        let mut file = archive.by_index(n)?;
        if file_regex.find(file.name()).unwrap().is_empty() {
            continue;
        }
        //println!("File name: {}", file.name());
        let mut s :String=String::from("");
        file.read_to_string(&mut s)?;
        //print!("{}",s);
        let mut buffer = String::with_capacity(1_000_000);
        s.lines()
            .map(|line| data_regex.captures(line).unwrap())
            .for_each(|captures| {
                for capture_name in data_regex.capture_names() {  
                    match capture_name {
                        None => (),
                        Some(capture_name) =>  {
                            let line = format!("{},{},{}\n",capture_name, &captures[capture_name],file_name);
                            buffer.push_str(&line);
                            //output_file.lock().unwrap().write_all(line.as_bytes()).expect("Cannot write file");
                        }
                    }
                }
        }
            );
        let mut file_handle = output_file.lock().unwrap();
        file_handle.write_all(buffer.as_bytes()).expect("Cannot write file");
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
    fs::create_dir_all(DATA_DIR)?;

    let file = File::create(&path)?;

    let mut zip = ZipWriter::new(file);

    zip.start_file("readme.txt", FileOptions::default())?;

    for n in 0..FILE_LINES {
        let line = format!("Hello, World! {}\n",n);
        zip.write_all(line.as_bytes())?;
    }
    zip.finish()?;
    //println!("Zip file [{}] created successfully!",path);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn extract_10_threads_100_files() -> Result<(), Box<dyn Error>>{
        extract_files(10,100)
    }


//#[test] 
//    fn extract_20_threads_50_files() -> Result<(), Box<dyn Error>>{
//        extract_files(20,50)
//    }
//#[test] 
//    fn extract_40_threads_25_files() -> Result<(), Box<dyn Error>>{
//        extract_files(20,25)
//    }
}

