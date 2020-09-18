use clap::{App, Arg};

mod parser;
use player;

fn main() {
  
  let args = App::new("tab-synth")
    .version("0.1")
    .about("WIP - play tabs from cli eventually..")
    .arg(Arg::with_name("generate-template")
           .short("g")
           .long("generate-template")
           .value_name("FILE")
           .takes_value(true)
           .default_value("./tab-template.txt")
           .required(true))
    .arg(Arg::with_name("play-file")
           .short("p")
           .long("play")
           .value_name("FILE")
           .takes_value(true)
           .required(true))
    .get_matches();

  //If --generate-template, then generate a template

  if args.is_present("generate-template") {
    let path = args.value_of("generate-template").unwrap();
    println!("generating file at: {}", path);
    parser::generate_template(path).unwrap();
  }

  if args.is_present("play-file") {

    let file = args.value_of("play-file").unwrap();


    let notes = parser::parse_file(file).unwrap();
    println!("playing {:?}", notes);
    player::play_track(notes, 400.);

    std::thread::sleep(std::time::Duration::from_secs(10));
  }



}