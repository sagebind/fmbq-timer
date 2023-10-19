use std::{
    sync::mpsc::{channel, Sender},
    thread,
};

use hound::WavSpec;
use ndk::audio::{AudioCallbackResult, AudioFormat, AudioStreamBuilder, AudioStreamState};

use crate::audio_player::AudioPlayer;

pub struct AAudioPlayer {
    join_handle: Option<thread::JoinHandle<()>>,
    sender: Sender<Message>,
}

impl AAudioPlayer {
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        let callback_sender = sender.clone();

        Self {
            sender,
            join_handle: Some(thread::spawn(move || {
                let (mut input, mut output) = crate::audio::queue::queue();

                let stream = AudioStreamBuilder::new()
                    .unwrap()
                    // .content_type(ndk::audio::AudioContentType::Sonification)
                    .format(AudioFormat::PCM_I16)
                    .channel_count(1)
                    .sample_rate(44100)
                    .data_callback(Box::new(move |_stream, buf, frames| {
                        let channels = 1; // item.spec.channels
                        let num_samples = frames as usize * channels as usize;
                        let buf: &mut [i16] = unsafe {
                            std::slice::from_raw_parts_mut(buf.cast::<i16>(), num_samples)
                        };

                        if output.read(buf) == buf.len() {
                            AudioCallbackResult::Continue
                        } else {
                            let _ = callback_sender.send(Message::Stop);
                            AudioCallbackResult::Stop
                        }
                    }))
                    .open_stream()
                    .unwrap();

                for message in receiver {
                    match message {
                        Message::Play { spec, data } => {
                            input.play_now(spec, data);

                            if !matches!(
                                stream.get_state(),
                                Ok(AudioStreamState::Started | AudioStreamState::Starting)
                            ) {
                                stream.request_start().unwrap();
                            }
                        }
                        Message::Stop => {
                            stream.request_stop().unwrap();
                        }
                        Message::Shutdown => {
                            stream.request_stop().unwrap();
                            break;
                        }
                    }
                }
            })),
        }
    }
}

impl Default for AAudioPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioPlayer for AAudioPlayer {
    fn play_audio(&self, spec: WavSpec, data: Vec<i16>) {
        self.sender.send(Message::Play { spec, data }).unwrap();
    }
}

impl Drop for AAudioPlayer {
    fn drop(&mut self) {
        if self.sender.send(Message::Shutdown).is_ok() {
            if let Some(handle) = self.join_handle.take() {
                handle.join().unwrap();
            }
        }
    }
}

#[derive(Clone, Debug)]
enum Message {
    Play { spec: WavSpec, data: Vec<i16> },
    Stop,
    Shutdown,
}
