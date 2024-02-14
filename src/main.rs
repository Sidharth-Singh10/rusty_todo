// use console::Term;
use std::env;
use std::fs::File;
// use std::fs::OpenOptions;
use std::io::{self,BufReader, Read};

fn open_file(filename: &str) -> Result<File, io::Error> {
    match File::open(filename) {
        Ok(file) =>Ok(file),
        Err(_) => {
            match File::create(filename) {
                Ok(file) => {
                    println!("New todo created");     
                    Ok(file)},
                Err(err) => Err(err),
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let  filename = &args[1];

    let file = match open_file(filename) {
        Ok(file) => file,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let command = &args[2];
    match &command[..] 
    {
        "list" => {
            // if let Err(err) = file.seek(SeekFrom::Start(0)) {
            //     println!("Error seeking to start of file: {}", err);
            //     return;
            // }

            let mut buf_reader = BufReader::new(&file);
            let mut contents = String::new();
            match buf_reader.read_to_string(&mut contents)
            {

                Ok(_) => {
                    let mut counter = 1;
                    for word in contents.split_whitespace() {
                        println!("{}.{}", counter, word);
                        counter += 1;
                    }
                },
                Err(err) => println!("Error reading file: {}", err),
            
            }
        }
        _ => println!("Unknown command: {}", command),
    }
}
