use hound::WavSpec;

pub trait AudioPlayer {
    fn play_audio(&self, spec: WavSpec, data: Vec<i16>);
}

pub struct NullAudioPlayer;

impl AudioPlayer for NullAudioPlayer {
    fn play_audio(&self, _spec: WavSpec, _data: Vec<i16>) {}
}

pub type Impl = crate::platform::android::audio_player::AAudioPlayer;

pub fn create() -> Impl {
    #[cfg(target_os = "android")]
    {
        return crate::platform::android::audio_player::AAudioPlayer::new();
    }

    #[cfg(not(target_os = "android"))]
    {
        NullAudioPlayer
    }
}
