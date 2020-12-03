use clap::{App, Arg};
use std::path::Path;

mod parser;
use player;

fn main() {
  let args = App::new("tab-player-cli")
    .version("0.1")
    .about("Play guitar tabs from the command line!")
    .arg(Arg::with_name("generate-template")
           .short("g")
           .long("generate-template")
           .value_name("FILENAME")
           .takes_value(true)
           .default_value("./tab-template.txt")
           .required(true))
    .arg(Arg::with_name("play-file")
           .short("p")
           .long("play")
           .takes_value(true)
           .number_of_values(2)
           .value_name("TAB_FILE")
           .value_name("SAMPLES_FOLDER")
           .required(true))
    .get_matches();

  if args.is_present("generate-template") {
    let path = args.value_of("generate-template").unwrap();
    println!("generating file at: {}", path);
    parser::generate_template(path).unwrap();
  }

  if args.is_present("play-file") {
    let values: Vec<&str> = args.values_of("play-file").unwrap().collect();
    let tab_file = values[0];
    let samples_folder = values[1];
    let notes = parser::parse_file(tab_file).unwrap();
    println!("Playing track, {} notes.", notes.len());
    player::play_track(notes, 200., Path::new(samples_folder));
  }
}