use std::env;

pub fn run() {
    // let args: Vec<String> = std::env::args().collect();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Bye~");
        return;
    }

    let command = args[1].clone();
    let name = "Brad";
    let status = "100%";

    // first : exe file name
    println!("Args: {:?} Command: {}", args, command);

    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("Invalid argument");
    }
}