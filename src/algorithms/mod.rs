//! Compression algorithm implementations
//!
//! This module contains specific compression algorithm implementations that build
//! on the core pattern engine and traits.

pub mod enhanced_ctw;
pub mod multi_pass;
pub mod practical_max;

pub use enhanced_ctw::EnhancedCTW;
pub use multi_pass::MultiPassCompressor;
pub use practical_max::PracticalMaxCompression;