use apps_boot::filereading::BootFile;
use std::env;

fn main() {
    let cli_args: Vec<String> = env::args().collect();
    if cli_args.len() < 3 {
        panic!("There must be at least 2 arguments")
    }
    let operation = &cli_args[1];
    let filename: &String = &cli_args[2];
    if operation == "run" {
        let binding = BootFile::Run(filename);
        binding.call().unwrap();
    }
    else if operation == "delete" {
        let binding = BootFile::Delete(filename);
        binding.call().unwrap();
    }
    else {
        panic!("Please select a valid operation: run / delete")
    };


}
