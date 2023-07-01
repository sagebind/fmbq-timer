// use rodio::{
//     source::{Source, Buffered},
//     Decoder, OutputStream, OutputStreamHandle, buffer::SamplesBuffer,
// };
use std::{
    collections::HashMap,
    io::{BufReader, Cursor},
};

use hound::{WavReader, WavSpec};

static WAV: &[u8] = include_bytes!("sounds/correct.wav");

#[derive(Clone, Debug)]
struct StaticWav {
    spec: WavSpec,
    samples: Vec<i16>,
}

impl StaticWav {
    fn new(bytes: &'static [u8]) -> Self {
        let reader = WavReader::new(Cursor::new(bytes)).unwrap();
        let spec = reader.spec();
        let samples = reader
            .into_samples::<i16>()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        Self { spec, samples }
    }
}

pub struct Player {
    // _output_stream: OutputStream,
    // output_stream_handle: OutputStreamHandle,
    sounds: HashMap<String, StaticWav>,
}

impl Player {
    pub fn new() -> Self {
        // let (output_stream, output_stream_handle) = OutputStream::try_default().unwrap();

        let mut sounds = HashMap::new();
        sounds.insert("correct".to_string(), StaticWav::new(WAV));

        Self {
            //     _output_stream: output_stream,
            //     output_stream_handle,
            sounds,
        }
    }

    pub fn play_timer_sound(&self) {
        let source = self.sounds.get("correct").unwrap();
        play_wav(source);
    }
}

// fn load_wav(bytes: &'static [u8]) -> Buffered<SamplesBuffer<f32>> {
//     let reader = BufReader::new(Cursor::new(bytes));
//     let source = Decoder::new(reader).unwrap().convert_samples::<f32>();

//     SamplesBuffer::new(source.channels(), source.sample_rate(), source.collect::<Vec<_>>()).buffered()
// }

fn play_wav(wav: &StaticWav) {
    #[cfg(target_os = "android")]
    {
        let wav = wav.clone();
        let mut index = 0;

        let stream = ndk::audio::AudioStreamBuilder::new().unwrap()
            // .content_type(ndk::audio::AudioContentType::Sonification)
            .format(ndk::audio::AudioFormat::PCM_I16)
            .channel_count(wav.spec.channels as _)
            .sample_rate(wav.spec.sample_rate as _)
            .data_callback(Box::new(move |s, buf, frames| {
                if index >= wav.samples.len() {
                    ndk::audio::AudioCallbackResult::Stop
                } else {
                    let num_samples = frames as usize * wav.spec.channels as usize;
                    let mut buf: &mut [i16] = unsafe {
                        std::slice::from_raw_parts_mut(buf.cast::<i16>(), num_samples)
                    };

                    for i in 0..buf.len() {
                        buf[i] = wav.samples.get(index + i).cloned().unwrap_or_default();
                    }
                    index += buf.len();

                    ndk::audio::AudioCallbackResult::Continue
                }
            }))
            .open_stream()
            .unwrap();

        stream.request_start().unwrap();
        std::thread::sleep_ms(2000);
        stream.request_stop().unwrap();
    }
}
