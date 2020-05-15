//! A Rust CAD System - A library for building 2D *Computer Aided Design*
//! applications.
//!
//! ## A Note on Conventions
//!
//! When using this crate you'll frequently need to work with multiple
//! coordinate spaces. To prevent accidentally mixing up vectors or points in
//! different coordinate spaces (see [the Mars Climate Orbiter][mco]), we use
//! [`euclid`]'s ability to "tag" a geometry primitive with something
//! representing the coordinate space it belongs to.
//!
//! For convenience we expose type aliases for the main coordinate space you'll
//! be using, [`DrawingSpace`].
//!
//! For more details on when each coordinate space is used, consult the docs for
//! [`DrawingSpace`] and [`CanvasSpace`].
//!
//! [mco]: https://en.wikipedia.org/wiki/Mars_Climate_Orbiter

#![forbid(unsafe_code)]

// #![deny(
//     missing_debug_implementations,
//     missing_copy_implementations,
//     missing_docs
// )]

pub mod components;
pub mod systems;
pub mod window;

pub use arcs_core::*;
