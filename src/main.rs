use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use walkdir::WalkDir;

// directory_parsing function is parsing the directory recusively for all the files with
// a given extension and then counting the number of lines in individual files and printing the path and number of lines.
fn directory_parsing(directory: String, extension: String) -> Result<(), String> {
    for entry in WalkDir::new(&directory).follow_links(true).into_iter() {
        let directory = Path::new(directory.as_str());
        // Check if the directory exists.
        if !directory.exists() {
            return Err(String::from("Directory does not exists!"));
        }
        // Check for valid directory, for example, it should give error when a file is given
        if !directory.is_dir() {
            return Err(String::from("This is not a valid directory!"));
        }
        match entry {
            Ok(ent) => {
                let file_name = ent.file_name();
                let path = ent.path();
                if path.extension() == Some(OsStr::new(extension.as_str())) {
                    let file = BufReader::new(File::open(&path).expect("Unable to open file!"));
                    println!(
                        "Path : {:?}\nFile NAme : {:?}\nNumber of lines in the file : {}",
                        path,
                        file_name,
                        file.lines().count()
                    );
                }
            }
            Err(err) => {
                println!("Error : {:?}", err);
            }
        }
    }
    return Ok(());
}

fn main() {
    match directory_parsing(
        String::from("test_dir"), // put a valid Path or directory
        String::from("json"),     // extension without '.'
    ) {
        Ok(res) => println!("Success!"),
        Err(err) => {
            println!("Error : {:?}", err);
        }
    }
}
