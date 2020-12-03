use rodio::{dynamic_mixer, source::Buffered, Decoder};
use std::time::Duration;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use rodio::{Source};
pub use pitch_calc::{LetterOctave, Letter};

type SampleData = (LetterOctave, Buffered<Decoder<BufReader<File>>>);

fn load_samples<P: AsRef<Path>>(samples_path: P) -> Vec<SampleData> {
  let mut samples_lib = Vec::new();

  let folder = samples_path.as_ref();
  let file = File::open(folder.join("e2d.wav")).unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::E, 2), source));
  let file = File::open(folder.join("f#2d.wav")).unwrap();
  // let file = File::open("./player/assets/electric-guitar/f#2d.wav").unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::Fsh, 2), source));
  let file = File::open(folder.join("g#2d.wav")).unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::Gsh, 2), source));
  let file = File::open(folder.join("a#2d.wav")).unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::Ash, 2), source));
  let file = File::open(folder.join("c3d.wav")).unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::C, 3), source));
  let file = File::open(folder.join("d3d.wav")).unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::D, 3), source));
  let file = File::open(folder.join("e3d.wav")).unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::E, 3), source));
  let file = File::open(folder.join("f#3d.wav")).unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::Fsh, 3), source));
  let file = File::open(folder.join("g#3d.wav")).unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::Gsh, 3), source));
  let file = File::open(folder.join("c4d.wav")).unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::C, 4), source));
  let file = File::open(folder.join("d4d.wav")).unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::D, 4), source));
  let file = File::open(folder.join("e4d.wav")).unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::E, 4), source));
  let file = File::open(folder.join("f#4d.wav")).unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::Fsh, 4), source));
  let file = File::open(folder.join("g#4d.wav")).unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::Gsh, 4), source));
  let file = File::open(folder.join("c5d.wav")).unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::C, 5), source));
  let file = File::open(folder.join("d5d.wav")).unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::D, 5), source));
  let file = File::open(folder.join("e5d.wav")).unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::E, 5), source));
  let file = File::open(folder.join("f#5d.wav")).unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::Fsh, 5), source));
  let file = File::open(folder.join("g#5d.wav")).unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::Gsh, 5), source));
  let file = File::open(folder.join("c6d.wav")).unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
  samples_lib.push((LetterOctave(Letter::C, 6), source));
  
  samples_lib
}

#[derive(Debug, Eq, PartialEq)]
pub struct Note {
  pitch: LetterOctave,
  beat_index: u32,
  beat_length: u32
}

impl Note {
  pub fn new(note: LetterOctave, beat_index: u32, beat_length: u32) -> Note {
    Note {
      pitch: note,
      beat_index: beat_index,
      beat_length: beat_length,
    }
  }
}

//Function blocks for enough time to play entire track
pub fn play_track<P: AsRef<Path>>(track: Vec<Note>, bpm: f32, samples_path: P){
  //shadow bpm to convert from quarter note beats to sixteenth note beats
  let bpm = bpm * 4.;
  let samples = load_samples(samples_path);

  let device = rodio::default_output_device().unwrap();
  let (controller, mixer) =
    dynamic_mixer::mixer(1, 44_100);

  let mut track_duration: u32 = 0;

  //TODO: BPM should be based on time signature.
  let beat_dur = Duration::from_millis((60000 as f32 / bpm) as u64);

  for note in track.iter() {
    let mut match_index: Option<usize> = None;
    for (i, sample) in samples.iter().rev().enumerate() {
      if sample.0 <= note.pitch {
        match_index = Some(i);
        let step_diff = note.pitch.step() - sample.0.step();
        let speed = (2 as f32).powf(step_diff / 12.);
        controller.add(
          sample.1.clone()
            .speed(speed)
            .take_duration(beat_dur * note.beat_length)
            .delay(note.beat_index as u32 * beat_dur)
        );
        let note_end = note.beat_index + note.beat_length;
        if note_end > track_duration {
          track_duration = note_end;
        }
        break;
      }
    }
    if match_index == None {
      panic!("note is not valid, lower than lowest sample! {:?}", note.pitch);
    }
  }
  let track_duration: Duration = beat_dur * track_duration;
  rodio::play_raw(&device, mixer.convert_samples());
  std::thread::sleep(track_duration);
}

#[cfg(test)]
mod tests {
  use super::*;
  use pitch_calc::letter::Letter::*;

  //put together notes to be played, play them
  #[ignore]
  #[test]
  fn play_test () {
    let note_vec = vec![
      Note::new(LetterOctave(E, 2), 0, 1),
      Note::new(LetterOctave(G, 2), 4, 1),
      Note::new(LetterOctave(E, 2), 8, 10),
    ];

    play_track(note_vec, 100., "./assets/electric-guitar/");
  }
}

