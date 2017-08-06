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
use blrustix::rustix_backend::WriteBackend;


fn main() {
    println!("I am all the beerlist you will ever need!");

    print_help();

    let mut backend = blrustix::build_persistent_backend(std::path::Path::new(".")).unwrap();

    println!("{:?}", backend.reload());

    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                println!("Line: {}", line);
                let _ = parse_line(line, &mut backend);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
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
    fn from(_: std::num::ParseIntError) -> Self {
        return CLIError::MyOwnError {
            name: "oh no!".to_string(),
        };
    }
}



fn current_time_seconds() -> u32 {
    return 123456789u32;
}

fn print_help() -> () {
    println!(
        "
Possible Commands:

//create user <name>
//create item <name> <price_cents: u32>
//create item <name> <price_cents: u32> <category>
//buy <user_id: u32> <item_id: u32>
//make bill <comment>
//delete item <id: u32>
//delete user <id: u32>
//print
//help
//exit"
    )
}


fn parse_line(
    line: String,
    backend: &mut blrustix::rustix_backend::RustixBackend<blrustix::persistencer::FilePersister>,
) -> Result<(), CLIError> {
    let v: Vec<&str> = line.trim().split(" ").collect();
    let v = v.into_iter()
        .filter(|&s| s.trim().len() > 0)
        .collect::<Vec<&str>>();
    match v.len() {
        1usize => {
            //println!("Only one element: {:?}", v);
            match v[0].as_ref() {
                "exit" => std::process::exit(0),
                "help" => Ok(print_help()),
                "print" => Ok(output_data(backend)),
                _ => Ok(println!("could not read text: {:?}", v[0])),
            }
        }
        2usize => {
            //println!("Two elements");
            match v[0] {
                _ => Ok(()),
            }
        }
        3usize => {
            //println!("Three elements");
            match v[0].as_ref() {
                "make" => match v[1].as_ref() {
                    "bill" => {
                        let ts = current_time_seconds();
                        backend.create_bill(
                            ts,
                            blrustix::datastore::UserGroup::AllUsers,
                            v[2].to_string(),
                        );
                        Ok(())
                    }
                    _ => Ok(()),
                },
                "delete" => match v[1].as_ref() {
                    "user" => {
                        let user_id = try!(v[2].parse::<u32>());
                        backend.delete_user(user_id);
                        Ok(())
                    }
                    "item" => {
                        let item_id = try!(v[2].parse::<u32>());
                        backend.delete_item(item_id);
                        Ok(())
                    }
                    _ => Ok(()),
                },
                "create" => match v[1].as_ref() {
                    "user" => {
                        backend.create_user(v[2].to_string());
                        Ok(())
                    }
                    _ => Ok(()),
                },
                "buy" => {
                    let user_id = try!(v[1].parse::<u32>());
                    let item_id = try!(v[2].parse::<u32>());

                    let ts = current_time_seconds();
                    backend.purchase(user_id, item_id, ts);
                    Ok(())
                }
                _ => Ok(()),
            }
        }
        4usize => {
            //println!("Four elements");
            if v[0] == "create" && v[1] == "item" {
                let name = v[2].to_string();
                let price_float = try!(v[3].parse::<u32>());
                backend.create_item(name, price_float, None);
            }
            Ok(())
            // ...
        }
        5usize => {
            //println!("Five elements");
            if v[0] == "create" && v[1] == "item" {
                let name = v[2].to_string();
                let price_float = try!(v[3].parse::<u32>());
                let category = v[4];
                backend.create_item(name, price_float, Some(category.to_string()));
            }
            Ok(())

        }
        _ => Ok(println!("Understood nothing, see vector = {:?}", v)),
    }
}

fn output_data(
    backend: &blrustix::rustix_backend::RustixBackend<blrustix::persistencer::FilePersister>,
) {
    println!("Output:\n{:#?}", backend);
}
