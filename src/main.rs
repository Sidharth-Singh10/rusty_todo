use console::style;
use std::env;
// use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, Read, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // let file = match open_file(filename) {
    //     Ok(file) => file,
    //     Err(err) => {
    //         println!("{}", err);
    //         return;
    //     }
    // };

    let command = &args[2];
    match &command[..] {
        "list" => {
            let file = match OpenOptions::new().read(true).open(filename) {
                Ok(file) => file,
                Err(_) => {
                    println!("No such todo created");
                    return;
                }
            };

            let mut buf_reader = BufReader::new(&file);
            let mut contents = String::new();
            match buf_reader.read_to_string(&mut contents) {
                Ok(_) => {
                    println!("{}", style(filename).bold().blue());
                    let mut counter = 1;
                    for word in contents.split_whitespace() {
                        println!("{}.{}", counter, word);
                        counter += 1;
                    }
                }
                Err(err) => println!("Error reading file: {}", err),
            }
        }

        "add" => {
            let file = match OpenOptions::new().append(true).open(filename) {
                Ok(file) => file,
                Err(_) => {
                    println!("No such todo created");
                    return;
                }
            };

            let mut buf_writer = BufWriter::new(file);
            let adds = &args[3..];
            for (i, argss) in adds.iter().enumerate() {
                if i > 0 {
                    writeln!(buf_writer).expect("Cannot write into file");
                }
                writeln!(buf_writer, "{}", argss).expect("Cannot write into file");
            }
            buf_writer.flush().expect("Unable to flush buffer");
        }

        "del" => {
            let file = OpenOptions::new()
                .read(true)
                .open(filename)
                .expect("Couldn't open the todo file");

            let mut buf_reader = BufReader::new(&file);
            let mut contents = String::new();

            buf_reader.read_to_string(&mut contents)
            .expect("Error reading file");

            let file = OpenOptions::new()
            .truncate(true)
            .write(true)
            .open(filename)
            .expect("Couldn't open the todo file");

            match buf_reader.read_to_string(&mut contents)
            {
                Ok(_) => 
                {
                    let mut buf_writer = BufWriter::new(file);
                    let dels = &args[3..];
                    for (_pos, word) in contents.split_whitespace().enumerate()
                    {
                        if dels.contains(&word.to_string())
                        {
                            continue;
                        }
                        
                        writeln!(buf_writer, "{}", word).expect("Cannot write into file");

                    }
                    buf_writer.flush().expect("Unable to flush buffer");
                }

                Err(err) => println!("Error reading file: {}", err),
            }
        }
        _ => println!("Unknown command: {}", command),
    }
}
