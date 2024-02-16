use console::style;
use std::env;
use std::process;
// use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, Read, Write};

fn read_file(filename: &str) -> std::fs::File {
    OpenOptions::new()
        .read(true)
        .open(filename)
        .expect("Couldn't open the todo file")
}
fn append_file(filename: &str) -> std::fs::File {
    OpenOptions::new()
        .append(true)
        .open(filename)
        .expect("Couldn't open the todo file")
}
fn write_file(filename: &str) -> std::fs::File {
    OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(filename)
        .expect("Couldn't open the todo file")
}
fn clear_file(filename: &str) -> std::fs::File {
    OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(filename)
        .expect("Couldn't open the todo file")
}

fn list(filename: &str) {
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
            println!("{}", style(filename).bold().cyan());
            let mut counter = 1;
            for word in contents.split_whitespace() {
                println!("{}.{}", style(counter).bold(), word);
                counter += 1;
            }
        }
        Err(err) => println!("Error reading file: {}", err),
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let command = &args[2];
    match &command[..] {
        "list" => {
            list(&filename);
        }

        "add" => {
            let file = append_file(&filename);

            let mut buf_writer = BufWriter::new(file);
            let adds = &args[3..];
            for (_i, argss) in adds.iter().enumerate() {
                // if i > 0 {
                //     writeln!(buf_writer).expect("Cannot write into file");
                // }
                writeln!(buf_writer, "{}", argss).expect("Cannot write into file");
            }
            buf_writer.flush().expect("Unable to flush buffer");

            list(&filename);
        }

        "rm" => {
            let dels = &args[3..];
            if dels.is_empty() {
                eprintln!("rm takes at least 1 argument");
                process::exit(1);
            }
            let file = read_file(&filename);

            let mut buf_reader = BufReader::new(&file);
            let mut contents = String::new();

            buf_reader
                .read_to_string(&mut contents)
                .expect("Error reading file");

            let file = write_file(&filename);

            match buf_reader.read_to_string(&mut contents) {
                Ok(_) => {
                    let mut buf_writer = BufWriter::new(file);

                    for (pos, word) in contents.split_whitespace().enumerate() {
                        if dels.contains(&word.to_string())
                            || dels.contains(&((&pos) + 1).to_string())
                        {
                            continue;
                        }

                        writeln!(buf_writer, "{}", word).expect("Cannot write into file");
                    }
                    buf_writer.flush().expect("Unable to flush buffer");
                }

                Err(err) => println!("Error reading file: {}", err),
            }
            list(&filename);
        }

        "done" => {
            let dones = &args[3..];
            if dones.is_empty() {
                eprintln!("rm takes at least 1 argument");
                process::exit(1);
            }
            let file = read_file(&filename);

            let mut buf_reader = BufReader::new(&file);
            let mut contents = String::new();

            buf_reader
                .read_to_string(&mut contents)
                .expect("Error reading file");

            let file = write_file(&filename);

            let mut buf_writer = BufWriter::new(file);

            for line in contents.lines() {
                let mut is_done = false;
                for word in line.split_whitespace() {
                    if dones.contains(&word.to_string()) {
                        is_done = true;
                        break;
                    }
                }

                if is_done {
                    writeln!(buf_writer, "{}", style(line).strikethrough().green())
                        .expect("Cannot write into file");
                } else {
                    writeln!(buf_writer, "{}", line).expect("Cannot write into file");
                }
            }

            buf_writer.flush().expect("Unable to flush buffer");

            list(&filename);
        }

        "clear" => {
            let _file = clear_file(&filename); //need better option
        }

        _ => println!("Unknown command: {}", command),
    }
}
