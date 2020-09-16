use std::fs;
use std::fs::File;
use std::io::prelude::*;

use itertools::Itertools;

const BEAT_WIDTH: u32 = 3;
const BEAT_OFFSET: u32 = 3;
const EMPTY_NOTE: &str = "--";
///TODO: Should be configurable
const BEATS_PER_MEASURE: usize = 16;
const MEASURE_SEPARATOR: char = '|';
const NOTE_SEPARATOR: char = ' ';
const HOLD_IDEN: &str = ">>";



//TODO: generate template should write to generic buffer instead of to a file directly

pub fn generate_template(save_file: &str) -> std::io::Result<()>{
  //TODO: take in path as arg, use it to create file with specific location
  let mut file = File::create(save_file)?;

  //TODO: configurable - should annotate if standard tuning?
  let standard_tuning = ["E4", "B3", "G3", "D3", "A2", "E2"];

  //TODO: configurable.
  let measures_per_row: usize = 2;
  let num_rows: usize = 2;

  //TODO: unhard code this - should be based on timing parameters.
  file.write_all(b"  1/4         2/4         3/4         4/4\n")?;
  
  for _ in 0..num_rows {
    //TODO: unhard code this - should be based on timing parameters.
    file.write_all(b"   |           |           |           |         ")?;
    for _ in 0..(measures_per_row - 1) { 
      file.write_all(b"  |           |           |           |")?;
    }
    file.write_all(b"\n")?;

    //For each string
    for open_note in standard_tuning.iter() {
      let mut line = String::new();
      line.push_str(*open_note);
      line.push(MEASURE_SEPARATOR);
      for _ in 0..measures_per_row {
        for _ in 0..BEATS_PER_MEASURE {
          line.push_str(EMPTY_NOTE);
          line.push(NOTE_SEPARATOR);
        }
        line.pop();
        line.push(MEASURE_SEPARATOR);
      }
      line.push('\n');
      file.write_all(line.as_bytes())?;
    }
    file.write_all(b"\n")?;
  }

  Ok(())
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum StringLetter {
  E4,
  B3,
  G3,
  D3,
  A2,
  E2
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Note {
  start_beat: u32, //Rhythm wise, where does the note start - units of sixteenth note.
  fret: u32,
  length: u32,
  string: StringLetter
}

impl Note {
  fn new(start_beat: u32, fret: u32, length: u32, string: StringLetter) -> Note {
    Note{
      start_beat: start_beat,
      fret: fret,
      length: length,
      string: string
    }
  }
}

pub fn play_file(play_file: &str) -> std::io::Result<()> {
  //First need to parse file into a format from which the notes can be applied to the samples
  // that is, a vec of {fret, start_beat, length} for each string
  let notes = parse_file(play_file)?;
  Ok(())
}

fn parse_file(file: &str) -> Result<Vec<Note>, std::io::Error> {
  let contents = fs::read_to_string(file)?;

  let mut parsed_notes: Vec<Note> =  Vec::new();

  for line in contents.lines() {
    //Check first char of line

    println!("parsing line: {}", line);

    // let substr = &line[0..2];

    //TODOD: for custom  instruments / arbitrary strings, this should read in any note 
    // eg A1, C3, F#4. need to make the section for string name one unit wider
    let letter = match line.chars().chunks(2).into_iter().next() {
      Some(c) => match c.collect_tuple().unwrap() {
        ('E','4') => Some(StringLetter::E4),
        ('B','3') => Some(StringLetter::B3),
        ('G','3') => Some(StringLetter::G3),
        ('D','3') => Some(StringLetter::D3),
        ('A','2') => Some(StringLetter::A2),
        ('E','2') => Some(StringLetter::E2),
        _ => None
        },
      None => None
    };
    let mut note_active = false;
    let mut read_fret: u32 = 0;
    let mut note_start: u32 = 0;
    let mut note_length: u32 = 1;

    if let Some(letter) = letter {
      //Found letter identifer at start of line - continue to parse line for notes

      //Seems to work, but don't understand well, not sure about how many copies are being made.
      for (i, s) in line.chars().skip(BEAT_OFFSET as usize).chunks(BEAT_WIDTH as usize).into_iter().enumerate() {
        //this gives the 3 char string that we can check for note
        let val: String = s.collect();
        println!("{}", val);
        if let Ok(fret) = val.trim().parse::<u32>() {

          if note_active {
            parsed_notes.push(Note::new(note_start, read_fret, note_length, letter));
            note_length = 1;
          }
          //fret found! add note
          note_active = true;
          read_fret = fret;
          //TODO: add offset to i for row
          note_start = i as u32;

          println!("index {}, fret {}", i, fret);
        } else if note_active && val.contains(HOLD_IDEN) {
          note_length += 1;
          //Don't need to do anything? 
        } else if note_active && val.contains(EMPTY_NOTE){
          parsed_notes.push(Note::new(note_start, read_fret, note_length, letter));
          note_length = 1;
          note_active = false;
        }
      }
    }
    
  }

  Ok(parsed_notes)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn gen_template_test() {
    let gold_string = fs::read_to_string("./test/gold-template.txt").unwrap();
    generate_template("./test/test-gen.txt").unwrap();
    let generated_string = fs::read_to_string("./test/test-gen.txt").unwrap();

    assert_eq!(gold_string, generated_string);
  }

  #[test]
  fn parser_test() {
    let correct_notes = vec![
      Note::new(4, 2, 1, StringLetter::G3),
      Note::new(0, 7, 6, StringLetter::A2),
      Note::new(6, 7, 2, StringLetter::A2),
      Note::new(8, 10, 3, StringLetter::A2),
      Note::new(11, 7, 3, StringLetter::A2),
      Note::new(14, 5, 2, StringLetter::A2),
    ];
    
    let notes = parse_file("./test/seven-nation-army.txt").unwrap();
    assert_eq!(notes, correct_notes);
  }
}