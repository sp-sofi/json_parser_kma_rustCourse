use anyhow::Ok;
use my_parser_sofia::*;
use my_parser_sofia::JSONValue;
//use clap::{App, Arg};
use clap::Parser as ParserClap;
use std::fs;

fn main() {
    if let Err(err) = start() {
        eprintln!("Error: {}", err);
    }


    // let unparsed_file = fs::read_to_string("data.json").expect("cannot read file");
    // let json: JSONValue = parse_json_file(&unparsed_file).expect("unsuccessful parse");
    // println!("{}", serialize_jsonvalue(&json));
} 

#[derive(ParserClap, Debug)]
#[command(author, version, about, long_about = None)]
//input arguments
struct Args {
    #[arg(short, long)]
    input_file: String,

    #[arg(short, long)]
    output_file: String,
}

pub fn start() -> Result<(), anyhow::Error> {
    let args = Args::parse(); //read args
    
    let content = fs::read_to_string(&args.input_file).expect("unsuccessful parse"); // read content from file
    let json: JSONValue = parse_json_file(&content).expect("unsuccessful parse"); // parse content from file
    fs::write(args.output_file, serialize_jsonvalue(&json).as_bytes()).expect("unsuccessful parse"); //write result to file           
            
    Ok(())
    
}
//.\target\debug\my_parser_sofia.exe --input-file a --output-file o
//.\target\debug\my_parser_sofia.exe --help



//cargo test -simple check
//cargo test -- --show-output 