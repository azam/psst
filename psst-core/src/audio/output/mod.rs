use std::fmt::Display;

use crate::{audio::source::AudioSource, error::Error};

#[cfg(feature = "cpal")]
pub mod cpal;
#[cfg(feature = "cubeb")]
pub mod cubeb;
#[cfg(feature = "pipewire")]
pub mod pipewire;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Backend {
    #[cfg(feature = "cpal")]
    Cpal,
    #[cfg(feature = "cubeb")]
    Cubeb,
    #[cfg(feature = "pipewire")]
    Pipewire,
}

pub const BACKENDS : &[Backend] = &[
    #[cfg(feature = "cpal")]
    Backend::Cpal,
    #[cfg(feature = "cubeb")]
    Backend::Cubeb,
    #[cfg(feature = "pipewire")]
    Backend::Pipewire,
];

impl Backend {
    pub fn open(&self) -> Result<Box<dyn AudioOutput>, Error> {
        match self {
            #[cfg(feature = "cpal")]
            Backend::Cpal => cpal::CpalOutput::open(),
            #[cfg(feature = "cubeb")]
            Backend::Cubeb => cubeb::CubebOutput::open(),
            #[cfg(feature = "pipewire")]
            Backend::Pipewire => pipewire::PipewireOutput::open(),
            #[allow(unreachable_patterns)]
            _ => panic!("no audio output backend is available"),
        }
    }
}

impl Default for Backend {
    fn default() -> Self {
        {
            #[cfg(feature = "cpal")]
            return Backend::Cpal;
            #[cfg(all(feature = "cubeb", not(feature = "cpal")))]
            return Backend::Cubeb;
            #[cfg(all(feature = "pipewire", not(feature = "cpal"), not(feature = "cubeb")))]
            return Backend::Pipewire;
        }
        panic!("no audio output backend is available");
    }
}

impl Display for Backend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "cpal")]
            Backend::Cpal => write!(f, "CPAL"),
            #[cfg(feature = "cubeb")]
            Backend::Cubeb => write!(f, "cubeb"),
            #[cfg(feature = "pipewire")]
            Backend::Pipewire => write!(f, "PipeWire"),
        }
    }
}

pub trait AudioOutput: Send + 'static {
    fn sink(&self) -> Box<dyn AudioSink>;
}

pub trait AudioSink: Send + 'static  {
    fn channel_count(&self) -> usize;
    fn sample_rate(&self) -> u32;
    fn set_volume(&self, volume: f32);
    fn play(&self, source: Box<dyn AudioSource>);
    fn pause(&self);
    fn resume(&self);
    fn stop(&self);
    fn close(&self);
}
