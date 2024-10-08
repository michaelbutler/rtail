//! A simple implementation of the `tail` command in Rust.
//! # Examples:
//! ```shell
//! $ cargo run -- -n 5 /etc/passwd
//! $ cargo run -- -n 5 < /etc/passwd
//! $ cargo run -- -c 3 < myfile.txt
//! ```

use clap::Parser;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, BufRead, BufReader, Write};
use std::vec;
use std::{io, process::ExitCode};

const BUFFER_SIZE: usize = 4096;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The number of lines to print
    #[arg(short, long, default_value_t = 10)]
    number: u32,

    /// The number of characters to print (optional)
    #[arg(short, long, conflicts_with = "number", default_value_t = 0)]
    chars: u32,

    /// The file to read from (optional)
    file: Option<String>,
}

enum Input {
    File(File),
    Stdin(io::Stdin),
}

// impl Input {
//     fn is_seekable(&self) -> bool {
//         match self {
//             Input::File(_) => true,
//             Input::Stdin(_) => false,
//         }
//     }
// }

fn tail(input: Input, num_lines: usize) -> io::Result<()> {
    match input {
        Input::File(file) => tail_seekable(file, num_lines),
        Input::Stdin(stdin) => tail_non_seekable(stdin, num_lines),
    }
}

fn tail_seekable(mut file: File, num_lines: usize) -> io::Result<()> {
    // Get the file size
    let file_size = file.seek(SeekFrom::End(0)).expect("File should be seekable");
    if file_size == 0 {
        return Ok(());  // Empty file
    }

    // Set a flag if we stripped the last LF or not.
    // Because if we did NOT, we need to make sure we don't add an LF when printing results.
    // This is to match the behavior of the native tail program.
    let mut last_char_buffer = [0u8; 1];
    file.seek(SeekFrom::End(-1)).expect("File should be seekable");
    file.read_exact(&mut last_char_buffer).expect("File should be readable");
    let stripped_last_lf = last_char_buffer[0] == b'\n';

    // Read BUFFER_SIZE or the file_size, whichever is smaller
    // Of course if the file size is less than the BUFFER_SIZE,
    // the whole file will be read in one loop iteration.
    let mut chunk_size: usize = BUFFER_SIZE.min(file_size as usize);
    let mut leftover: Vec<u8> = Vec::new();
    let mut gathered_lines: Vec<String> = Vec::new();
    let mut current_pointer = file_size as usize;
    let mut first_iter = true;
    let mut last_iter = false;

    loop {
        // Handle case where we reverse back to the beginning of the file
        if chunk_size >= current_pointer {
            chunk_size = current_pointer;
            last_iter = true;
        }

        // Start position of read operation
        current_pointer = current_pointer - chunk_size; 

        // Now Seek Backwards
        _ = file.seek(SeekFrom::Start(current_pointer as u64));
        let mut chunk: Vec<u8> = vec![0; chunk_size];
        let read_result = file.read_exact(&mut chunk);

        match read_result {
            Ok(()) => {
                // println!("Read {} bytes...", chunk.len());
            },
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                return Err(e);
            }
        }

        // If the last char of the entire file is a newline we should ignore it
        if first_iter {
            first_iter = false;
            if chunk.last() == Some(&b'\n') {
                chunk.pop();
            }
        }

        chunk.append(&mut leftover);

        let mut chunk_lines: Vec<&[u8]> = chunk.split(|&b| b == b'\n').collect();

        loop {
            let mut line = chunk_lines.pop().unwrap().to_vec();
            if chunk_lines.is_empty() {
                // TODO: find better way to prepend "line" to "leftover"
                line.append(&mut leftover);
                leftover.append(&mut line);
                break;
            }
            let strline = String::from_utf8(line).unwrap();
            gathered_lines.push(strline);
            if gathered_lines.len() >= num_lines {
                break;
            }
        }

        if gathered_lines.len() >= num_lines {
            break;
        }

        if last_iter {
            // Handle case when we're at beginning of file and haven't reached desired number of lines yet.
            // So we need to include the first line of the file.
            gathered_lines.push(String::from_utf8(leftover).unwrap());
            break;
        }
    }

    for line in gathered_lines.iter().rev().take(gathered_lines.len() - 1) {
        println!("{}", line);
    }

    if let Some(last_line) = gathered_lines.first() {
        print!("{}", last_line);
        if stripped_last_lf {
            println!();
        }
    }

    Ok(())
}

fn tail_non_seekable(stdin: io::Stdin, num_lines: usize) -> io::Result<()> {
    let mut circular_buffer = Vec::with_capacity(num_lines);
    let mut current_index = 0;

    let reader = BufReader::new(stdin);
    for line in reader.lines() {
        let line = line?;
        if circular_buffer.len() < num_lines {
            circular_buffer.push(line);
        } else {
            circular_buffer[current_index] = line;
            current_index = (current_index + 1) % num_lines;
        }
    }

    // Print lines in correct order
    if !circular_buffer.is_empty() {
        let mut index = if circular_buffer.len() < num_lines {
            0
        } else {
            current_index
        };

        for _ in 0..circular_buffer.len() {
            println!("{}", circular_buffer[index]);
            index = (index + 1) % circular_buffer.len();
        }
    }

    Ok(())
}

fn print_chars<R: BufRead>(mut reader: R, n: u32) {
    let n = n as usize;
    let buffer_size = n.min(4096);
    let mut buffer = vec![0; buffer_size];
    let mut bytes_remaining = n;

    while bytes_remaining > 0 {
        let bytes_to_read = bytes_remaining.min(buffer.len());
        match reader.read(&mut buffer[..bytes_to_read]) {
            Ok(0) => break,
            Ok(bytes_read) => {
                // This has the possibilty of printing invalid UTF-8 characters
                // But it should not crash.
                io::stdout().write_all(&buffer[..bytes_read]).unwrap();
                bytes_remaining -= bytes_read;
            }
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                return;
            }
        }
    }
}

fn main() -> ExitCode {
    let args = Args::parse();
    let numlines = args.number;
    let chars = args.chars;

    if numlines == 0 {
        eprintln!("Invalid number of lines to print: {}", numlines);
        return ExitCode::FAILURE;
    }

    let input = match args.file {
        Some(f) => {
            let file = File::open(f).unwrap();
            Input::File(file)
        },
        None => Input::Stdin(io::stdin()),
    };

    match chars {
        0 => match tail(input, numlines as usize) {
            Ok(_) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("Error: {}", e);
                return ExitCode::FAILURE;
            }
        },
        _ => {
            eprint!("Not supported yet!!");
            ExitCode::FAILURE
        },
        // _ => print_chars(&mut br, chars),
    }
}
