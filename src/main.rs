use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 0 {
        println!("no command");
        return;
    }
    let command = &args[0];
    match command.as_str() {
        "init" => println!("aaaaaa"),
        _ => println!("rit: '#{{ {} }}' is not a rit command.", command),
    }
}
