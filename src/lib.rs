#![doc = include_str!("../README.md")]

mod data;
mod engine;
mod error;
mod inflect_call;
mod numbers;
mod text;
mod types;

pub use engine::Engine;
pub use error::Error;
pub use types::{Classical, Comparison, Count, Gender, JoinOptions, NumberToWordsOptions};
