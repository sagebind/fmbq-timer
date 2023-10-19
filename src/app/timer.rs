use crate::{audio_player::AudioPlayer, sounds::SoundLibrary};
use appstorage::Storage;
use std::{
    sync::mpsc::{channel, Receiver, RecvTimeoutError, Sender},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};
use triple_buffer::{triple_buffer, Input, Output};

static DEFAULT_RESOLUTION: Duration = Duration::from_millis(100);
static SPIN_THRESHOLD: Duration = Duration::from_millis(15);

/// This is the actual core timer implementation.
pub struct Timer {
    messages: Sender<Message>,
    state: Output<State>,
    _worker_thread: JoinHandle<()>,
}

/// Describes the possible states of a timer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    /// The timer is not currently running.
    Stopped,

    /// The timer is currently counting down from the latest time started.
    Running {
        /// The amount of time remaining.
        remaining: Duration,

        /// The timer value currently being counted down from.
        total: Duration,
    },
}

impl State {
    pub fn is_running(&self) -> bool {
        matches!(self, Self::Running { .. })
    }
}

impl Timer {
    /// Create a new timer.
    pub fn new(storage: Storage) -> Self {
        Self::with_update_resolution(DEFAULT_RESOLUTION, storage)
    }

    pub fn with_update_resolution(update_resolution: Duration, storage: Storage) -> Self {
        let (sender, receiver) = channel();
        let (input, output) = triple_buffer(&State::Stopped);

        Self {
            messages: sender,
            state: output,
            _worker_thread: thread::spawn(move || {
                let mut worker = Worker {
                    messages: receiver,
                    state: input,
                    expiration: None,
                    duration: Duration::ZERO,
                    update_resolution,
                    audio_player: crate::audio_player::create(),
                    storage,

                    #[cfg(target_os = "android")]
                    wake_lock_guard: None,
                };

                worker.run();
            }),
        }
    }

    /// Check if the timer is currently running.
    pub fn is_running(&mut self) -> bool {
        self.state().is_running()
    }

    /// Get the current timer state.
    ///
    /// This fetches the latest state reported from the background thread.
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

/// Background worker
struct Worker {
    messages: Receiver<Message>,
    state: Input<State>,
    expiration: Option<Instant>,
    duration: Duration,
    update_resolution: Duration,
    audio_player: crate::audio_player::Impl,
    storage: Storage,

    #[cfg(target_os = "android")]
    wake_lock_guard: Option<android_wakelock::Guard<'static>>,
}

/// A message used to communicate with the worker.
enum Message {
    Start(Instant, Duration),
    Reset,
    TestAudio,
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
                    self.state.write(State::Running {
                        remaining: time_remaining,
                        total: self.duration,
                    });

                    if let Some(time) = time_remaining.checked_sub(SPIN_THRESHOLD) {
                        // Sleep until the timer expires, the state needs
                        // updated based on the timer resolution, or a new
                        // message is sent.
                        let sleep_time = time.min(self.update_resolution);

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

    #[cfg(target_os = "android")]
    fn wake_lock() -> &'static android_wakelock::WakeLock {
        use std::sync::OnceLock;

        static WAKE_LOCK: OnceLock<android_wakelock::WakeLock> = OnceLock::new();

        WAKE_LOCK.get_or_init(|| android_wakelock::partial("fmbqtimer:timer").unwrap())
    }

    fn handle(&mut self, message: Message) {
        match message {
            Message::Start(start_time, duration) => {
                let end_time = start_time + duration;
                self.expiration = Some(end_time);
                self.duration = duration;
                self.state.write(State::Running {
                    remaining: duration,
                    total: duration,
                });

                #[cfg(target_os = "android")]
                {
                    // Prevent the device from sleeping when a timer starts.
                    if self.wake_lock_guard.is_none() {
                        self.wake_lock_guard = Some(Self::wake_lock().acquire().unwrap());
                    }
                }
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
        log::info!("timer expired");
        self.reset();
    }

    fn reset(&mut self) {
        self.expiration = None;
        self.state.write(State::Stopped);

        // Allow the device to sleep again when the timer is not running.
        #[cfg(target_os = "android")]
        drop(self.wake_lock_guard.take());
    }

    fn start_alarm_audio(&self) {
        let sound_name = self
            .storage
            .get::<String>("sound")
            .unwrap_or_else(|| "correct".to_owned());

        if let Some((spec, data)) = SoundLibrary::get().get_by_name(&sound_name) {
            self.audio_player.play_audio(spec, data.into_inner());
        }
    }
}
