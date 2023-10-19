use hound::WavSpec;

pub trait AudioPlayer {
    fn play_audio(&self, spec: WavSpec, data: Vec<i16>);
}

pub struct NullAudioPlayer;

impl AudioPlayer for NullAudioPlayer {
    fn play_audio(&self, _spec: WavSpec, _data: Vec<i16>) {}
}

#[cfg(target_os = "android")]
pub type Impl = crate::platform::android::audio_player::AAudioPlayer;

#[cfg(not(target_os = "android"))]
pub type Impl = self::cpal::CpalAudioPlayer;

pub fn create() -> Impl {
    #[cfg(target_os = "android")]
    {
        return crate::platform::android::audio_player::AAudioPlayer::new();
    }

    #[cfg(not(target_os = "android"))]
    {
        self::cpal::CpalAudioPlayer::new()
    }
}

#[cfg(not(target_os = "android"))]
mod cpal {
    use crate::audio::queue::AudioQueue;

    use super::AudioPlayer;
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    use cpal::{Sample, SampleFormat, StreamConfig, StreamError, SupportedStreamConfig};
    use hound::WavSpec;
    use std::sync::Mutex;

    pub struct CpalAudioPlayer {
        stream: cpal::Stream,
        queue: Mutex<AudioQueue>,
    }

    impl CpalAudioPlayer {
        pub fn new() -> Self {
            let host = cpal::default_host();
            let device = host
                .default_output_device()
                .unwrap();

            let (mut queue, mut output) = crate::audio::queue::queue();

            let config = device.supported_output_configs().unwrap()
            .filter(|config| config.channels() == 1)
            .filter(|config| config.sample_format() == SampleFormat::I16)
            .next()
            .unwrap()
            .with_sample_rate(cpal::SampleRate(44100));
            let stream = device.build_output_stream(
                &config.config(),
                move |data: &mut [i16], _: &cpal::OutputCallbackInfo| {
                    output.read(data);
                    // react to stream events and read or write stream data here.
                },
                move |err| {
                    // react to errors here.
                },
                None // None=blocking, Some(Duration)=timeout
            ).unwrap();

            Self { stream, queue: Mutex::new(queue) }
        }
    }

    impl AudioPlayer for CpalAudioPlayer {
        fn play_audio(&self, spec: WavSpec, data: Vec<i16>) {
            self.queue.lock().unwrap().play_now(spec, data);
        }
    }
}
