//! Sound library management.

use hound::{WavReader, WavSpec};
use std::{collections::BTreeMap, io::Cursor, sync::OnceLock};

pub struct SoundLibrary {
    sounds: BTreeMap<String, StaticWav>,
}

impl SoundLibrary {
    pub fn get() -> &'static Self {
        static INSTANCE: OnceLock<SoundLibrary> = OnceLock::new();

        INSTANCE.get_or_init(Self::load)
    }

    pub fn load() -> Self {
        let mut sounds = BTreeMap::new();

        static WAV_DIR: include_dir::Dir =
            include_dir::include_dir!("$CARGO_MANIFEST_DIR/src/sounds/wav");

        for file in WAV_DIR.files() {
            let name = file
                .path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned();
            sounds.insert(name, StaticWav::new(file.contents()));
        }

        Self { sounds }
    }

    pub fn get_by_name(&self, name: &str) -> Option<(WavSpec, Cursor<Vec<i16>>)> {
        self.sounds
            .get(name)
            .map(|sound| (sound.spec, Cursor::new(sound.samples.clone())))
    }

    pub fn get_names(&self) -> Vec<String> {
        self.sounds.keys().cloned().collect()
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        let library = SoundLibrary::load();

        assert!(library.sounds.len() > 2);
        assert!(library.sounds.contains_key("correct"));
    }
}
