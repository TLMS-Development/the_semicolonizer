use std::io::{stdin, Read};

mod args;
fn main() {
    let command = args::get_command();
    let matches = command.get_matches();
    let input_arg = matches.get_many::<String>("input");
    let mut inputs = Vec::new();

    if input_arg.is_some() {
        let input_arg = input_arg
            .unwrap()
            .map(String::from)
            .collect::<Vec<String>>();
        inputs.extend_from_slice(&input_arg);
    } else {
        let mut input = String::new();
        let result = stdin().read_to_string(&mut input);

        match result {
            Ok(_) => {
                let input_arg = input
                    .trim()
                    .replace(' ', "\n")
                    .lines()
                    .map(String::from)
                    .collect::<Vec<String>>();
                inputs.extend_from_slice(&input_arg);
            }
            Err(e) => {
                eprintln!("Error reading from stdin: {}", e);
            }
        }
    }

    for line in inputs {
        print!("{};", line);
    }
}
