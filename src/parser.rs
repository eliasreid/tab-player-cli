use std::fs;
use std::fs::File;
use std::io::prelude::*;

use itertools::Itertools;

use player::{Note, LetterOctave, Letter};
use player;

const BEAT_WIDTH: u32 = 3;
const BEAT_OFFSET: u32 = 4;
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
  let standard_tuning = ["E4 ", "B3 ", "G3 ", "D3 ", "A2 ", "E2 "];

  //TODO: configurable.
  let measures_per_row: usize = 2;
  let num_rows: usize = 2;

  //TODO: unhard code this - should be based on timing parameters.
  file.write_all(b"   1/4         2/4         3/4         4/4\n")?;
  
  for _ in 0..num_rows {
    //TODO: unhard code this - should be based on timing parameters.
    file.write_all(b"    |           |           |           |         ")?;
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


pub fn parse_file(file: &str) -> Result<Vec<Note>, std::io::Error> {
  let contents = fs::read_to_string(file)?;

  let mut parsed_notes: Vec<Note> =  Vec::new();

  for line in contents.lines() {
    //Check first char of line

    println!("parsing line: {}", line);

    let mut note_active = false;
    let mut read_fret: u32 = 0;
    let mut note_start: u32 = 0;
    let mut note_length: u32 = 1;
    if let Some(open_note) = parse_note_name(&line.chars().take(3).collect::<String>()) {
      //found a note name for this line
      for (i, s) in line.chars().skip(BEAT_OFFSET as usize).chunks(BEAT_WIDTH as usize).into_iter().enumerate() {
        //this gives the 3 char string that we can check for identifier
        let val: String = s.collect();
        if let Ok(fret) = val.trim().parse::<u32>() {

          if note_active {
            //could convert to step, then convert back to letter octave
            let note = open_note.to_step() + (read_fret as f32).into();
            parsed_notes.push(Note::new(note.into(), note_start, note_length));
            note_length = 1;
          }
          //fret found! add note
          note_active = true;
          read_fret = fret;
          //TODO: add offset to i for row
          note_start = i as u32;
        } else if note_active && val.contains(HOLD_IDEN) {
          note_length += 1;
          //Don't need to do anything?
        } else if note_active && val.contains(EMPTY_NOTE){
          let note = open_note.to_step() + (read_fret as f32).into();
          parsed_notes.push(Note::new(note.into(), note_start, note_length));
          note_length = 1;
          note_active = false;
        }
      }
    }



  }

  Ok(parsed_notes)
}

fn parse_note_name(string: &str) -> Option<LetterOctave> {
  let string = string.chars().collect_vec();
  if string.len() > 3 || string.len() < 2 {
    return None;
  }

  let base_letter = match string[0] {
    'A' | 'a' => Letter::A,
    'B' | 'b' => Letter::B,
    'C' | 'c' => Letter::C,
    'D' | 'd' => Letter::D,
    'E' | 'e' => Letter::E,
    'F' | 'f' => Letter::F,
    'G' | 'g' => Letter::G,
     _  => return None
  };

  if let Some(octave) = string[1].to_digit(10) {
    //simple case, letter followed by number
    return Some(LetterOctave(base_letter, octave as i32));
  }
  //Make sure string is long enough to have an accidental
  if string.len() == 3 {
    if let Some(octave) = string[2].to_digit(10) {
      return match string[1] {
        '♭' | 'b' => Some(LetterOctave(base_letter - 1, octave as i32)),
        '♯' | '#' => Some(LetterOctave(base_letter + 1, octave as i32)),
        _ => None,
      };
    }
  }
  None
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn letteroctave_from_string() {
    use Letter::*;
    assert_eq!(parse_note_name("E4").unwrap(), LetterOctave(E, 4));
    assert_eq!(parse_note_name("E#4").unwrap(), LetterOctave(F, 4));
    assert_eq!(parse_note_name("Cb4").unwrap(), LetterOctave(B, 4));

    assert_eq!(parse_note_name("A2").unwrap(), LetterOctave(A,2));
    assert_eq!(parse_note_name("F8").unwrap(), LetterOctave(F, 8));
    assert_eq!(parse_note_name("C♯3").unwrap(), LetterOctave(Csh, 3));
    assert_eq!(parse_note_name("C#3").unwrap(), LetterOctave(Csh, 3));
    assert_eq!(parse_note_name("B♭3").unwrap(), LetterOctave(Bb, 3));
    assert_eq!(parse_note_name("Bb3").unwrap(), LetterOctave(Bb, 3));
    assert_eq!(parse_note_name("Bb3 "), None);
    assert_eq!(parse_note_name("E"), None);
    assert_eq!(parse_note_name("Ef"), None);
    assert_eq!(parse_note_name("2f5"), None);
    assert_eq!(parse_note_name("B#e"), None);
    assert_eq!(parse_note_name("23"), None);
  }

  #[test]
  fn gen_template_test() {
    let gold_string = fs::read_to_string("./test/gold-template.txt").unwrap();
    generate_template("./test/test-gen.txt").unwrap();
    let generated_string = fs::read_to_string("./test/test-gen.txt").unwrap();

    assert_eq!(gold_string, generated_string);
  }

  #[test]
  fn parser_test() {
    use Letter::*;
    let correct_notes = vec![
      Note::new(LetterOctave(E, 3), 0, 6),
      Note::new(LetterOctave(E, 3), 6, 2),
      Note::new(LetterOctave(G, 3), 8, 3),
      Note::new(LetterOctave(E, 3), 11, 3),
      Note::new(LetterOctave(D, 3), 14, 2),

      Note::new(LetterOctave(C, 3), 16, 8),
      Note::new(LetterOctave(B, 2), 24, 8),


    ];

    let notes = parse_file("./test/seven-nation-army.txt").unwrap();
    assert_eq!(notes, correct_notes);
  }
}