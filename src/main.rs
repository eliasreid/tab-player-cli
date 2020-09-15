use std::fs::File;
use std::io::BufReader;
use rodio::{Source, play_once};

use rodio::dynamic_mixer;
use std::time::Duration;

use clap::{App, Arg};

mod parser;
use parser::generate_template;

fn main() {

  let args = App::new("tab-synth")
    .version("0.1")
    .about("WIP - play tabs from cli")
    .arg(Arg::with_name("generate-template")
           .short("g")
           .long("generate-template")
           .value_name("FILE")
           .takes_value(true)
           .default_value("./tab-template.txt")
           .required(true))
    .get_matches();

  //If --generate-template, then generate a template

  if args.is_present("generate-template") {
    let path = args.value_of("generate-template").unwrap();
    generate_template(path);
    return;
  }


  let device = rodio::default_output_device().unwrap();

  let (controller, mixer) =
    dynamic_mixer::mixer(1, 44_100);

  //They loops over tracks (which would be like strings, but not really)
  //has file, "step" info and volume

  //get file for given track

  let file1 = File::open("/home/elias/sampler/samples/guitar/e2d.wav").unwrap();
  //store source in buffer, in addition to returning it
  let source1 = rodio::Decoder::new(BufReader::new(file1)).unwrap().buffered();

  controller.add(source1.clone());
  controller.add(source1.clone().delay(Duration::from_millis(1000)));
  controller.add(source1.clone().delay(Duration::from_millis(1500)));

  let file2 = File::open("/home/elias/sampler/samples/guitar/g#2d.wav").unwrap();
  //store source in buffer, in addition to returning it
  let source2 = rodio::Decoder::new(BufReader::new(file2)).unwrap().buffered();

  controller.add(source2.clone().delay(Duration::from_millis(100)));
  //then for each soure (instrument, file. eg. snare drum), they iterate over the steps
  //source.clone.amplify.delay
  //delay is what allows to add subsequently

  rodio::play_raw(&device, mixer.convert_samples());
  loop{} // so that we don't exit before hearing sound
}