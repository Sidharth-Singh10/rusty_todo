use console::style;
use std::env;
use std::fs;
use std::fs::read_dir;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;
use std::process;

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

fn new_file(filename: &str) {
    File::create(filename).expect("coudnt create todo");
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
            if contents.is_empty() {
                println!("EMPTY");
                process::exit(1);
            }

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
fn set_path() {
    if let Ok(home_dir) = env::var("HOME") {
        let mut path = Path::new(&home_dir).to_path_buf();
        path.push("todos");
        if !path.exists() {
            fs::create_dir(&path).expect("Failed to create todos directory");
            println!("Created todos directory at: {:?}", path);
        } else {
            println!("Todos directory already exists at: {:?}", path);
        }
    } else {
        println!("Couldn't get home directory");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        let mut path = String::new();
        if let Ok(home_dir) = env::var("HOME") {
            path = format!("{}/todos", home_dir);
        } else {
            println!("Couldn't get home directory");
        }
        if let Ok(entries) = read_dir(&path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(name) = entry.file_name().to_str() {
                        println!("{}", name);
                    }
                }
            }
        } else {
            println!("Failed to read directory.");
        }
        process::exit(1);
    }

    let filename = &args[1];

    if filename == "set" {
        set_path();
        process::exit(1);
    }

    let mut path = String::new();

    if let Ok(home_dir) = env::var("HOME") {
        path = format!("{}/todos/{}", home_dir, filename);
    } else {
        println!("Couldn't get home directory");
    }

    let command = &args[2];
    match &command[..] {
        "list" => {
            list(&path);
        }

        "add" => {
            let file = append_file(&path);

            let mut buf_writer = BufWriter::new(file);
            let adds = &args[3..];
            for (_i, argss) in adds.iter().enumerate() {
                // if i > 0 {
                //     writeln!(buf_writer).expect("Cannot write into file");
                // }
                writeln!(buf_writer, "{}", argss).expect("Cannot write into file");
            }
            buf_writer.flush().expect("Unable to flush buffer");

            list(&path);
        }

        "rm" => {
            let dels = &args[3..];
            if dels.is_empty() {
                eprintln!("rm takes at least 1 argument");
                process::exit(1);
            }
            let file = read_file(&path);

            let mut buf_reader = BufReader::new(&file);
            let mut contents = String::new();

            buf_reader
                .read_to_string(&mut contents)
                .expect("Error reading file");

            let file = write_file(&path);

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
            list(&path);
        }

        "done" => {
            let dones = &args[3..];
            if dones.is_empty() {
                eprintln!("rm takes at least 1 argument");
                process::exit(1);
            }
            let file = read_file(&path);

            let mut buf_reader = BufReader::new(&file);
            let mut contents = String::new();

            buf_reader
                .read_to_string(&mut contents)
                .expect("Error reading file");

            let file = write_file(&path);

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

            list(&path);
        }

        "clear" => {
            let _file = clear_file(&path); //need better option
        }
        "new" => {
            new_file(&path);
        }

        _ => println!("Unknown command: {}", command),
    }
}
