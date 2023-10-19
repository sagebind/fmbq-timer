use hound::WavSpec;
use triple_buffer::{Input, Output};

pub fn queue() -> (AudioQueue, Receiver) {
    let (input, output) = triple_buffer::triple_buffer(&PlayingItem::silence());

    (AudioQueue { input }, Receiver { output })
}

pub struct AudioQueue {
    input: Input<PlayingItem>,
}

pub struct Receiver {
    output: Output<PlayingItem>,
}

impl AudioQueue {
    pub fn play_now(&mut self, spec: WavSpec, data: Vec<i16>) {
        let item = PlayingItem {
            spec,
            data,
            cursor: 0,
        };

        self.input.write(item);
    }
}

impl Receiver {
    pub fn read(&mut self, buf: &mut [i16]) -> usize {
        self.output.update();
        let item = self.output.output_buffer();

        if item.cursor < item.data.len() {
            let src = &item.data[item.cursor..];
            let amount = src.len().min(buf.len());

            let (dst, silence_dst) = buf.split_at_mut(amount);
            dst.copy_from_slice(&src[..amount]);
            silence_dst.fill(0);

            item.cursor += amount;

            amount
        } else {
            buf.fill(0);
            0
        }
    }
}

#[derive(Clone, Debug)]
struct PlayingItem {
    spec: WavSpec,
    data: Vec<i16>,
    cursor: usize,
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
            cursor: 0,
        }
    }
}
