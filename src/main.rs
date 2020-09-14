use synth::Synth;
use pitch_calc::{Letter, LetterOctave};
use portaudio as pa;

const CHANNELS: i32 = 1;
const FRAMES: u32 = 64;
const SAMPLE_HZ: f64 = 44_100.0;

// Currently supports i8, i32, f32.
pub type AudioSample = f32;
pub type Input = AudioSample;
pub type Output = AudioSample;

fn main() {

  let mut synth = {
    use synth::{Point, Oscillator, oscillator, Envelope};

    let amp_env = Envelope::from(vec![
      Point::new(0.0, 1.0, 0.0),
      Point::new(0.3, 0.5, 0.0),
      Point::new(1.0, 0.0, 0.0),
    ]);
    let freq_env = Envelope::from(vec![
      Point::new(0.0, 1.0, 0.0),
    ]);

    let oscillator = Oscillator::new(
      oscillator::waveform::Sine,
      amp_env,
      freq_env,
      ());

    Synth::retrigger(())
      .oscillator(oscillator)
      .duration(5000.0)
      .base_pitch(LetterOctave(Letter::C, 1).hz())
  };

  let note = LetterOctave(Letter::C, 1).hz();
  let note_velocity = 1.0;
  synth.note_on(note, note_velocity);
  let note_duration = 4.0;
  let mut is_note_off = false;

  let mut timer: f64 = 0.0;

  let mut prev_time = None;

  let callback = move | pa::OutputStreamCallbackArgs{buffer, time, ..}| {
    let buffer: &mut [[f32; CHANNELS as usize]] = sample::slice::to_frame_slice_mut(buffer).unwrap();
    sample::slice::equilibrium(buffer);

    synth.fill_slice(buffer, SAMPLE_HZ as f64);
    if timer < 6.0 {
      let last_time = prev_time.unwrap_or(time.current);
      let dt = time.current - last_time;
      timer += dt;

      if timer > note_duration {
        if !is_note_off {
          synth.note_off(note);
          is_note_off = true;
        }
      }
      pa::Continue
    } else {
      pa::Complete
    }
  };

  let pa = pa::PortAudio::new().unwrap();
  let settings = pa.default_output_stream_settings::<f32>(CHANNELS, SAMPLE_HZ, FRAMES).unwrap();

  let mut stream = pa.open_non_blocking_stream(settings ,callback).unwrap();

  stream.start().unwrap();

  while let Ok(true) = stream.is_active() {
    std::thread::sleep(std::time::Duration::from_millis(16));
  }


}
