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
                let (mut input, mut output) = triple_buffer::triple_buffer(&PlayingItem::silence());

                let stream = AudioStreamBuilder::new()
                    .unwrap()
                    // .content_type(ndk::audio::AudioContentType::Sonification)
                    .format(AudioFormat::PCM_I16)
                    .channel_count(2)
                    .sample_rate(44100)
                    .data_callback(Box::new(move |s, buf, frames| {
                        output.update();
                        let item = output.output_buffer();

                        if item.index >= item.data.len() {
                            let _ = callback_sender.send(Message::Stop);
                            AudioCallbackResult::Stop
                        } else {
                            let num_samples = frames as usize * item.spec.channels as usize;
                            let buf: &mut [i16] = unsafe {
                                std::slice::from_raw_parts_mut(buf.cast::<i16>(), num_samples)
                            };

                            for i in 0..buf.len() {
                                buf[i] = item.data.get(item.index + i).cloned().unwrap_or_default();
                            }
                            item.index += buf.len();

                            AudioCallbackResult::Continue
                        }
                    }))
                    .open_stream()
                    .unwrap();

                for message in receiver {
                    match message {
                        Message::Play(item) => {
                            input.write(item);

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
        let item = PlayingItem {
            spec,
            data,
            index: 0,
        };

        self.sender.send(Message::Play(item)).unwrap();
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

enum Message {
    Play(PlayingItem),
    Stop,
    Shutdown,
}

#[derive(Clone, Debug)]
struct PlayingItem {
    spec: WavSpec,
    data: Vec<i16>,
    index: usize,
}

impl PlayingItem {
    fn silence() -> Self {
        Self {
            spec: WavSpec {
                channels: 2,
                sample_rate: 44100,
                bits_per_sample: 16,
                sample_format: hound::SampleFormat::Int,
            },
            data: vec![],
            index: 0,
        }
    }
}
