use crate::{audio_player::AudioPlayer, sounds::SoundLibrary};
use std::{
    sync::mpsc::{channel, Receiver, RecvTimeoutError, Sender},
    thread,
    time::{Duration, Instant},
};
use triple_buffer::{Input, Output};

static RESOLUTION: Duration = Duration::from_millis(100);
static SPIN_THRESHOLD: Duration = Duration::from_millis(15);

/// This is the actual core timer implementation.
pub struct Timer {
    messages: Sender<Message>,
    state: Output<State>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Running(Duration, Duration),
    Stopped,
}

enum Message {
    Start(Instant, Duration),
    Reset,
    TestAudio,
}

impl Timer {
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        let (input, output) = triple_buffer::triple_buffer(&State::Stopped);

        let mut worker = Worker {
            messages: receiver,
            state: input,
            expiration: None,
            duration: Duration::ZERO,
            audio_player: crate::audio_player::create(),
        };

        thread::spawn(move || worker.run());

        Self {
            messages: sender,
            state: output,
        }
    }

    pub fn state(&mut self) -> State {
        *self.state.read()
    }

    /// Start the timer with a new duration. If the timer is already running
    /// then it will first reset.
    pub fn start(&mut self, duration: Duration) {
        self.messages
            .send(Message::Start(Instant::now(), duration))
            .unwrap();
    }

    /// Reset the timer now.
    pub fn reset(&mut self) {
        self.messages.send(Message::Reset).unwrap();
    }

    /// Start playing the timer sound now without affecting the timer state.
    ///
    /// This method is asynchronous and will return immediately.
    pub fn test_audio(&mut self) {
        self.messages.send(Message::TestAudio).unwrap();
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

/// Background worker
struct Worker {
    messages: Receiver<Message>,
    state: Input<State>,
    expiration: Option<Instant>,
    duration: Duration,
    audio_player: crate::audio_player::Impl,
}

impl Worker {
    fn run(&mut self) {
        loop {
            if let Some(expiration) = self.expiration {
                // Active timer is running, check the time remaining.
                let now = Instant::now();
                let time_remaining = expiration.saturating_duration_since(now);

                // Timer expired
                if time_remaining.is_zero() {
                    self.fire();
                } else {
                    // Timer still running, update the state.
                    self.state
                        .write(State::Running(time_remaining, self.duration));

                    if let Some(time) = time_remaining.checked_sub(SPIN_THRESHOLD) {
                        // Sleep until the timer expires, the state needs
                        // updated based on the timer resolution, or a new
                        // message is sent.
                        let sleep_time = time.min(RESOLUTION);

                        match self.messages.recv_timeout(sleep_time) {
                            Ok(message) => self.handle(message),

                            Err(RecvTimeoutError::Timeout) => (),

                            // Timer dropped
                            Err(RecvTimeoutError::Disconnected) => return,
                        }
                    } else {
                        // If we're close to the expiration, revert to a spin
                        // loop to wait out the remaining time. Mobile thread
                        // schedulers can be pretty aggressive about sleeping
                        // threads to improve battery life which can result in a
                        // wakeups way overshooting the deadline.
                        self.spin_wait(expiration);
                    }
                }
            } else {
                // No timer running, check for or wait for a new message.
                if let Ok(message) = self.messages.recv() {
                    self.handle(message);
                } else {
                    // Timer dropped
                    return;
                }
            }
        }
    }

    fn handle(&mut self, message: Message) {
        match message {
            Message::Start(start_time, duration) => {
                let end_time = start_time + duration;
                self.expiration = Some(end_time);
                self.duration = duration;
                self.state.write(State::Running(duration, duration));
            }
            Message::Reset => self.reset(),
            Message::TestAudio => self.start_alarm_audio(),
        }
    }

    fn spin_wait(&self, deadline: Instant) {
        loop {
            thread::yield_now();

            if Instant::now() >= deadline {
                break;
            }
        }
    }

    fn fire(&mut self) {
        self.start_alarm_audio();
        self.reset();
    }

    fn reset(&mut self) {
        self.expiration = None;
        self.state.write(State::Stopped);
    }

    fn start_alarm_audio(&self) {
        let (spec, data) = SoundLibrary::get().get_any();
        self.audio_player.play_audio(spec, data.into_inner());
    }
}
