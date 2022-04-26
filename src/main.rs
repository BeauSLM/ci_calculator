use std::io::{ self, Write, Stdin, Stdout };
use std::str::FromStr;

struct IOHandler {
    stdin: Stdin,
    stdout: Stdout,
}

impl Default for IOHandler {
    fn default() -> Self {
        IOHandler {
            stdin: io::stdin(),
            stdout: io::stdout(),
        }
    }
}

impl IOHandler {
    fn read_then_parse<T: FromStr>(&mut self) -> Result<T, T::Err>
    {
        let mut input = String::new();
        // if reading/writing not happening, def not recoverable lul
        self.stdout.flush().unwrap();
        self.stdin.read_line(&mut input).unwrap();

        input.trim().parse()
    }
}

fn main() {
    println!("CI calculator");
    let mut io: IOHandler = IOHandler::default();

    loop {
        print!("what is the first number?: ");
        let num1: f32 = match io.read_then_parse() {
            Ok(n) => n,
            _ => {
                eprintln!("Please enter a valid number");
                continue;
            }
        };

        print!("what is the second number?: ");
        let num2: f32 = match io.read_then_parse() {
            Ok(n) => n,
            _ => {
                eprintln!("Please enter a valid number");
                continue;
            }
        };

        print!("what operation would you like to do? [+-*/]:");
        let operator: char = match io.read_then_parse() {
            Ok(n) => n,
            _ => {
                eprintln!("Please enter a valid number");
                continue;
            }
        };

        if !"+-*/".contains(operator) {
            println!("unknown operator");
            continue;
        }

        let result = match operator {
            '+' => num1 + num2,
            '-' => num1 - num2,
            '*' => num1 * num2,
            '/' => num1 / num2,
            _ => unreachable!(), // we checked if the operator was invalid already
        };

        println!("the result of {} {} {} = {}", num1, operator, num2, result);
    }
}
