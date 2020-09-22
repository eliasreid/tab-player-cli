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

//TODO: generate template should write to generic buffer instead of to a file directly (better for testing)

pub fn generate_template(save_file: &str) -> std::io::Result<()>{
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

struct StringParseState {
  open_note: LetterOctave,
  note_active: bool,
  note_start: u32,
  note: LetterOctave,
  // note_length: u32 //May not need to track note length.
}

impl StringParseState {
  
  fn new(open_note: LetterOctave) -> StringParseState {
    StringParseState {
      open_note: open_note,
      note_active: false,
      note_start: 0,
      note: LetterOctave(Letter::C, 4),
    }
  }

  fn new_note_at(&mut self, beat: u32, fret: u32) {
    self.note = LetterOctave::from(self.open_note.to_step() + (fret as f32).into());
    self.note_start = beat;
    self.note_active = true;
  }

  fn end_note(&mut self, end_index: u32) -> Option<Note> {
    if self.note_active {
      self.note_active = false;
      Some(Note::new(self.note, self.note_start, end_index - self.note_start))
    } else{
      None
    }
  }
}

pub fn parse_file(file: &str) -> Result<Vec<Note>, std::io::Error> {
  let contents = fs::read_to_string(file)?;
  let mut parsed_notes: Vec<Note> =  Vec::new();

  //store note tracking state for each string
  let mut string_states = Vec::new();
  let mut string_index: usize = 0;

  let mut row = 0;
  let mut beats_in_line: u32 = 0;
  let mut row_beat_offest: u32 = 0;

  for line in contents.lines() {
    if let Some(open_note) = parse_letter_octave(&line.chars().take(3).collect::<String>()) {
      //for the first row, populate the string list with a StringParseState for each string
      if row == 0 {
        string_states.push(StringParseState::new(open_note));
      } else if string_index < string_states.len() {
        //Checks that the strings match on subsequent rows. If not abort
        //TODO: Better error handling here.
        assert_eq!(string_states[string_index].open_note, open_note);

      }
      //iterate over beats in the line
      for (i, s) in line.chars().skip(BEAT_OFFSET as usize).chunks(BEAT_WIDTH as usize).into_iter().enumerate() {
        //TODO: figure out a way to not make a copy here? (Should be able to get a "view" into 3 chars of the string)
        let chunk: String = s.collect();
        let beat_index = row_beat_offest + i as u32;

        if let Ok(fret) = chunk.trim().parse::<u32>() {
          //If the string was already tracking a note, then end it, and add to note list
          if let Some(note) = string_states[string_index].end_note(beat_index){
            parsed_notes.push(note);
          }
          string_states[string_index].new_note_at(beat_index, fret);
        } else if chunk.contains(EMPTY_NOTE){
          if let Some(note) = string_states[string_index].end_note(beat_index){
            parsed_notes.push(note);
          }
        }
        //Don't have to handle the ">>" case explicitly. Just assume the note keeps going
      }
      string_index += 1;
      beats_in_line = (line.len() as u32 - BEAT_OFFSET) / BEAT_WIDTH;
    } else {
      //Found a line without a string name at the start
      if !string_states.is_empty() && string_index != 0 {
        row += 1;
        row_beat_offest += beats_in_line;
      }
      string_index = 0;
    }
  }

  //Check if there's any other notes that didn't explicitly end
  for string_state in string_states.iter_mut() {
    if let Some(note) = string_state.end_note(row_beat_offest) {
      parsed_notes.push(note);
    }
  }
  
  Ok(parsed_notes)
}

fn parse_letter_octave(string: &str) -> Option<LetterOctave> {
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
    assert_eq!(parse_letter_octave("E4").unwrap(), LetterOctave(E, 4));
    assert_eq!(parse_letter_octave("E#4").unwrap(), LetterOctave(F, 4));
    assert_eq!(parse_letter_octave("Cb4").unwrap(), LetterOctave(B, 4));

    assert_eq!(parse_letter_octave("A2").unwrap(), LetterOctave(A,2));
    assert_eq!(parse_letter_octave("F8").unwrap(), LetterOctave(F, 8));
    assert_eq!(parse_letter_octave("C♯3").unwrap(), LetterOctave(Csh, 3));
    assert_eq!(parse_letter_octave("C#3").unwrap(), LetterOctave(Csh, 3));
    assert_eq!(parse_letter_octave("B♭3").unwrap(), LetterOctave(Bb, 3));
    assert_eq!(parse_letter_octave("Bb3").unwrap(), LetterOctave(Bb, 3));
    assert_eq!(parse_letter_octave("Bb3 "), None);
    assert_eq!(parse_letter_octave("E"), None);
    assert_eq!(parse_letter_octave("Ef"), None);
    assert_eq!(parse_letter_octave("2f5"), None);
    assert_eq!(parse_letter_octave("B#e"), None);
    assert_eq!(parse_letter_octave("23"), None);
  }

  #[test]
  fn gen_template_test() {
    let gold_string = fs::read_to_string("./test/gold-template.txt").unwrap();
    generate_template("./test/test-gen.txt").unwrap();
    let generated_string = fs::read_to_string("./test/test-gen.txt").unwrap();

    println!("gen template test");
    for zipped in generated_string.lines().zip_longest(gold_string.lines()) {
      let (gen_line, gold_line) = zipped.both().unwrap();
      assert_eq!(gen_line, gold_line);
    }
  }

  #[test]
  fn one_line() {
    use Letter::*;
    let correct_notes = vec![
      Note::new(LetterOctave(E, 3), 0, 6),
      Note::new(LetterOctave(E, 3), 6, 2),
      Note::new(LetterOctave(G, 3), 8, 3),
      Note::new(LetterOctave(E, 3), 11, 3),
      Note::new(LetterOctave(D, 3), 14, 2),
      Note::new(LetterOctave(C, 3), 16, 8),
    ];

    let notes = parse_file("./test/seven-nation-army-one-line.txt").unwrap();
    assert_eq!(notes, correct_notes);
  }
  #[test]
  fn multi_line() {
    use Letter::*;
    let correct_notes = vec![
      Note::new(LetterOctave(E, 3), 0, 6),
      Note::new(LetterOctave(E, 3), 6, 2),
      Note::new(LetterOctave(G, 3), 8, 3),
      Note::new(LetterOctave(E, 3), 11, 3),
      Note::new(LetterOctave(D, 3), 14, 2),

      Note::new(LetterOctave(C, 3), 16, 8),
      Note::new(LetterOctave(B, 2), 24, 8),

      Note::new(LetterOctave(E, 3), 32, 6),
      Note::new(LetterOctave(E, 3), 38, 2),
      Note::new(LetterOctave(G, 3), 40, 3),
      Note::new(LetterOctave(E, 3), 43, 3),
      Note::new(LetterOctave(D, 3), 46, 2),

      Note::new(LetterOctave(C, 3), 48, 3),
      Note::new(LetterOctave(D, 3), 51, 3),
      Note::new(LetterOctave(C, 3), 54, 2),
      Note::new(LetterOctave(B, 2), 56, 8),
    ];

    let notes = parse_file("./test/seven-nation-army-full.txt").unwrap();
    for note in notes.iter() {
      println!("{:?}",note);
    }
    assert_eq!(notes, correct_notes);
  }
}