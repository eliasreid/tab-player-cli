use std::fs::File;
use std::io::prelude::*;
const BEAT_WIDTH: usize = 2;
const EMPTY_NOTE: &str = "--";
///TODO: Should be configurable
const BEATS_PER_MEASURE: usize = 16;
const MEASURE_SEPARATOR: char = '|';
const NOTE_SEPARATOR: char = ' ';

pub fn generate_template() -> std::io::Result<()>{
  ///TODO: take in path as arg, use it to create file with specific location
  let mut file = File::create("testfile.txt")?;

  ///TODO: configurable - should annotate if standard tuning?
  let standard_tuning = ['e', 'B', 'G', 'D', 'A', 'E'];

  ///TODO: configurable.
  let measures_per_row: usize = 2;
  let num_rows: usize = 2;

  //Before anything, write some times steps to help guide
  let mut time_fractions = String::from(" ");
  //TODO: unhard code this - should be based on timing parameters.
  file.write_all(b" 1/4         2/4         3/4         4/4\n");
  
  for _ in 0..num_rows {
    //TODO: unhard code this - should be based on timing parameters.
    file.write_all(b"  |           |           |           |         ");
    for _ in 0..(measures_per_row - 1) { 
      file.write_all(b"  |           |           |           |");
    }
    file.write_all(b"\n");

    //For each string
    for open_note in standard_tuning.iter() {
      let mut line = String::new();
      line.push(*open_note);
      line.push(MEASURE_SEPARATOR);
      for measure in 0..measures_per_row {
        for _ in 0..BEATS_PER_MEASURE {
          line.push_str(EMPTY_NOTE);
          line.push(NOTE_SEPARATOR);
        }
        line.pop();
        line.push(MEASURE_SEPARATOR);
      }
      line.push('\n');
      file.write_all(line.as_bytes());
    }
    file.write_all(b"\n");
  }

  Ok(())
}