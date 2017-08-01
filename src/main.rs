#[warn(unused_imports)]
#[warn(unused_variables)]
#[warn(unused_must_use)]

extern crate rustyline;

#[macro_use]
extern crate quick_error;

extern crate blrustix;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::io;
use std::num::ParseIntError;
use std::num::ParseFloatError;


fn main() {
    println!("Hello, world!");

    let mut backend = blrustix::default::build_transient_backend();

    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                println!("Line: {}", line);
                parse_line(line, &backend);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
}


quick_error! {
    #[derive(Debug)]
    pub enum CLIError {
        /// Parser Error
        ParseInt(err: ParseIntError) {}
        /// IO Error
        Io(err: io::Error) {}
        /// Readline Error
        Readline(err: ReadlineError) {}
        /// other
        MyOwnError{name: String}
    }
}

impl std::convert::From<std::num::ParseIntError> for CLIError {
    fn from(t: std::num::ParseIntError) -> Self {
        return CLIError::MyOwnError{name : "oh no!".to_string()};
    }
}




//create user <name>
//create item <name> <price_cents>
//create item <name> <price_cents> <category>
//make bill <comment>
//delete item <id>
//delete user <id>
//print
//help
//exit

fn parse_line(line: String, mut backend: &blrustix::rustix_backend::RustixBackend<blrustix::persistencer::TransientPersister>) -> Result<(), CLIError> {
    let v: Vec<&str> = line.trim().split(" ").collect();
    let v = v.into_iter().filter(|&s|s.trim().len() > 0).collect::<Vec<&str>>();
    match v.len() {
        1usize => {
            println!("Only one element: {:?}", v);
            match v[0].as_ref() {
                "exit" => Ok(()),
                "help" => Ok(()),
                "print" => Ok(output_data(backend)),
                _ => Ok(println!("could not read text: {:?}", v[0])),
            }
        },
        2usize => {
            println!("Two elements");
            match v[0] {_ => Ok(()),}
        },
        3usize => {
            println!("Three elements");
            let (letter, x1, x2) = (v[0], v[1], v[2]);
            match letter {
                "make" => Ok(()),
                "delete" => Ok(()),
                "create" => Ok(()),
                _ => Ok(()),
            }
        },
        4usize => {
            println!("Four elements");
            let (letter, x1, x2, x3) = (v[0], v[1], v[2], v[3]);
            if letter == "create" && x1 == "item" {
                let name = v[2];
                let price_float = try!(v[3].parse::<u32>());
            }
            Ok(())
            // ...
        },
        5usize => {
            println!("Five elements");
            let (letter, x1, x2, x3, x4) = (v[0], v[1], v[2], v[3], v[4]);
            if letter == "create" && x1 == "item" {
                let name = v[2];
                let price_float = try!(v[3].parse::<u32>());
                let category = v[4];
            }
            Ok(())

        },
        _ => Ok(println!("Understood nothing, see vector = {:?}", v)),
    }
}

fn output_data(backend: &blrustix::rustix_backend::RustixBackend<blrustix::persistencer::TransientPersister>) {
    println!("Output:\n{:#?}", backend);
}
