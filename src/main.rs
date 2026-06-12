mod utils;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        utils::help();
        std::process::exit(0)
    }
    let mut count = 0;

    loop {
        if count >= args.len() {
            break;
        };

        count += 1;
    }

    match args[1].as_str() {
        "save" => utils::save(args),
        "ls" => utils::ls(),
        "rm" => utils::rm(args),
        "h" => utils::help(),
        "help" => utils::help(),
        "find" => utils::find(args),
        _ => {
            println!("Unknown argument {}. 'See snip help'.", args[1])
        }
    }
}
