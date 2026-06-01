use day2_lab4_result_parser::parse_command;

fn main() {
    for line in ["SET motor 3", "STOP motor", "SET pump high"] {
        match parse_command(line) {
            Ok(command) => println!("ok: {command:?}"),
            Err(err) => println!("error: {err:?}"),
        }
    }
}
