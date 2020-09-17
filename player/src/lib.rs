use rodio::{dynamic_mixer, source::Buffered, Decoder};
use std::time::Duration;
use std::io::BufReader;
use std::fs::File;

use rodio::{Source};

use pitch_calc::{LetterOctave, Letter};
use pitch_calc::perc::Perc;

use std::collections::HashMap;

type SampleData = (LetterOctave, Buffered<Decoder<BufReader<File>>>);

fn load_samples() -> Vec<SampleData> {
  let mut samples_lib = Vec::new();

  let file = File::open("./assets/electric-guitar/e2d.wav").unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::E, 2), source));
  let file = File::open("./assets/electric-guitar/f#2d.wav").unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::Fsh, 2), source));
  let file = File::open("./assets/electric-guitar/g#2d.wav").unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::Gsh, 2), source));
  let file = File::open("./assets/electric-guitar/a#2d.wav").unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::Ash, 2), source));
  let file = File::open("./assets/electric-guitar/c3d.wav").unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::C, 3), source));
  let file = File::open("./assets/electric-guitar/d3d.wav").unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::D, 3), source));
  let file = File::open("./assets/electric-guitar/e3d.wav").unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::E, 3), source));
  let file = File::open("./assets/electric-guitar/f#3d.wav").unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::Fsh, 3), source));
  let file = File::open("./assets/electric-guitar/g#3d.wav").unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::Gsh, 3), source));
  let file = File::open("./assets/electric-guitar/c4d.wav").unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::C, 4), source));
  let file = File::open("./assets/electric-guitar/d4d.wav").unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::D, 4), source));
  let file = File::open("./assets/electric-guitar/e4d.wav").unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::E, 4), source));
  let file = File::open("./assets/electric-guitar/f#4d.wav").unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::Fsh, 4), source));
  let file = File::open("./assets/electric-guitar/g#4d.wav").unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::Gsh, 4), source));
  let file = File::open("./assets/electric-guitar/c5d.wav").unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::C, 5), source));
  let file = File::open("./assets/electric-guitar/d5d.wav").unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::D, 5), source));
  let file = File::open("./assets/electric-guitar/e5d.wav").unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::E, 5), source));
  let file = File::open("./assets/electric-guitar/f#5d.wav").unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::Fsh, 5), source));
  let file = File::open("./assets/electric-guitar/g#5d.wav").unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::Gsh, 5), source));
  let file = File::open("./assets/electric-guitar/c6d.wav").unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::C, 6), source));
  

  samples_lib
}

//How will I pass in the note data - structure that will be output by parser.

//For each "note":
// note (letter octave, parser can handle converting open string + fret to actual note value)
// beat index (integer number that can be multiplied by arbitrary bpm)
// note length (same units as index)

pub struct Note {
  note: LetterOctave,
  //for now, 16th notes, but can be more flexible.
  beat_index: u32,
  beat_length: u32
}

impl Note {
  pub fn new(note: LetterOctave, beat_index: u32, beat_length: u32) -> Note {
    Note {
      note: note,
      beat_index: beat_index,
      beat_length: beat_length,
    }
  }
}


type BPM = f32;

pub fn play_track(track: Vec<Note>, bpm: f32){
  //to avoid loading the samples on each "play_track", could have a Player struct that
  //holds the samples data, with member functions for playing things.
  let samples = load_samples();

  let device = rodio::default_output_device().unwrap();
  let (controller, mixer) =
    dynamic_mixer::mixer(1, 44_100);

  controller.add(samples[0].1.clone());
  // beat x, y beats per minute

  // x / y = z minutes
  //z * 60000 = z' ms

  let beat_dur = Duration::from_millis((60000 as f32 / bpm) as u64);

  for note in track.iter() {
    println!("note: {:?}", note.note);
    //how to go from beats per minute, to a constant that can be multiple by beat index?

    //find note that matches
    //add sample that matches the given note
    let mut found_match = false;
    for sample in samples.iter() {
      if note.note == sample.0 {
        found_match = true;
        controller.add(sample.1.clone().delay(note.beat_index * beat_dur));
      }
    }
    if found_match {
      println!("found matching note for {:?}", note.note);
    } else {
      println!("note not founds for {:?}", note.note);
    }

    // controller.add()
  }
  rodio::play_raw(&device, mixer.convert_samples());


}



#[cfg(test)]
mod tests {
  use super::*;
  use pitch_calc::letter::Letter::*;

  //put together notes to be played, play them
  #[ignore]
  #[test]
  fn play_test () {
    //initialize data structure of thing I want to play...
    let mut note_vec = vec![
      Note::new(LetterOctave(E, 2), 0, 1),
      Note::new(LetterOctave(D, 3), 1, 1),
      Note::new(LetterOctave(E, 3), 2, 1),
    ];

    play_track(note_vec, 100.);
    std::thread::sleep(Duration::from_secs(10));
  }

  //Loads samples from disk, and plays each of them 1 second apart.
  //should hear all the distinct notes
  #[ignore]
  #[test]
  fn load() {
    let lib = load_samples();

    let device = rodio::default_output_device().unwrap();
    let (controller, mixer) =
      dynamic_mixer::mixer(1, 44_100);

    for (i, sample_data) in lib.iter().enumerate() {
      controller.add(sample_data.1.clone().delay(Duration::from_secs(i as u64)));
    }

    rodio::play_raw(&device, mixer.convert_samples());
    std::thread::sleep(Duration::from_secs(25));
  }

  #[ignore]
  #[test]
  fn playground() {
    let device = rodio::default_output_device().unwrap();

    let (controller, mixer) =
      dynamic_mixer::mixer(1, 44_100);
  
    //They loops over tracks (which would be like strings, but not really)
    //has file, "step" info and volume
  
    //get file for given track
  
    let e2file = File::open("./assets/electric-guitar/e2d.wav").unwrap();
    //store source in buffer, in addition to returning it
    let e2source = rodio::Decoder::new(BufReader::new(e2file)).unwrap().buffered();
  
    //Note length entil I figure something else out - take_duration will do
    //Pitch modification - probably better to not get too fancy, just speed up the samples appropriately.

    controller.add(
      e2source.clone()
      .speed(2.)
      .take_duration(Duration::from_secs(1))
    );

    controller.add(
      e2source.clone()
      .speed(1.5)
      // .take_duration(std::time::Duration::from_secs(1))
    );

    // controller.add(source1.clone().delay(std::time::Duration::from_secs(2)));

    //fade out isn't great, means fade out after n time.
    // maybe ok for now

    //take(n) to iterate over first n iterations

    //WANT TO
    //Take N samples (based on note length)
    //fade out a few samples at the end.

    // controller.add(source1.clone().fade_out(std::time::Duration::from_secs(1)).delay(std::time::Duration::from_secs(2)));
    // controller.add(source1.clone().delay(Duration::from_millis(1000)).amplify(0.3));
    // controller.add(source1.clone().delay(Duration::from_millis(1500)).amplify(0.3));
  
    let file2 = File::open("./assets/electric-guitar/g#2d.wav").unwrap();
    //store source in buffer, in addition to returning it
    let source2 = rodio::Decoder::new(BufReader::new(file2)).unwrap().buffered();
  
    // controller.add(source2.clone().delay(Duration::from_millis(100)).amplify(0.3));
    //then for each soure (instrument, file. eg. snare drum), they iterate over the steps
    //source.clone.amplify.delay
    //delay is what allows to add subsequently
  
    rodio::play_raw(&device, mixer.convert_samples());

    //enough to hear the sound for now.
    std::thread::sleep(Duration::from_secs(5));
  }
}

