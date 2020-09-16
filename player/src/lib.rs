use rodio::{dynamic_mixer, source::Buffered, Decoder};
use std::time::Duration;
use std::io::BufReader;
use std::fs::File;

use rodio::{Source};

use pitch_calc::{LetterOctave};
use pitch_calc::perc::Perc;

use std::collections::HashMap;

type SamplesLib = HashMap<LetterOctave, Buffered<Decoder<BufReader<File>>>>;

//Load samples into some mode global hashmap of letter values to 
pub fn load_samples() -> SamplesLib {
  let mut samples_lib = HashMap::new();

  //Load each file, create hashmap entry.

  

  samples_lib
}




#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn pitches() {

  }

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

