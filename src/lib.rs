//! Whatlang is a Rust library to detect(regonize) natural languages.
//! Apart from it, the library also recognizes scripts (writing system).
//! Every language and script are represented by determined list of enums.
//!
//! # Examples
//!
//! Using `detect` function:
//!
//! ```
//! use whatlang::{detect, Lang, Script};
//!
//! let text = "Ĉu vi ne volas eklerni Esperanton? Bonvolu! Estas unu de la plej bonaj aferoj!";
//! let info = detect(text).unwrap();
//! assert_eq!(info.lang(), Lang::Epo);
//! assert_eq!(info.script(), Script::Latin);
//!
//! // Confidence is in the range from 0 to 1.
//! assert_eq!(info.confidence(), 1.0);
//! assert!(info.is_reliable());
//! ```
//!
//! Using `Detector` with specified denylist or allowlist:
//!
//! ```
//! use whatlang::{Detector, Lang};
//!
//! let whitelist = vec![Lang::Eng, Lang::Rus];
//!
//! // You can also create detector using with_denylist function
//! let detector = Detector::with_allowlist(whitelist);
//! let lang = detector.detect_lang("There is no reason not to learn Esperanto.");
//! assert_eq!(lang, Some(Lang::Eng));
//! ```
//!
mod alphabets;
mod combined;
mod core;
mod error;
mod lang;
mod scripts;
mod trigrams;
mod utils;

#[cfg(feature = "dev")]
pub mod dev;

pub use crate::core::{detect, detect_lang, Detector, Info};
pub use crate::lang::Lang;
pub use crate::scripts::{detect_script, Script};
