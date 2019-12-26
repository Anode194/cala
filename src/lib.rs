//! ![Cala](https://libcala.github.io/logo.svg)
//!
//! [![docs.rs](https://docs.rs/cala/badge.svg)](https://docs.rs/cala)
//! [![build status](https://api.travis-ci.com/libcala/cala.svg?branch=master)](https://travis-ci.com/libcala/cala)
//! [![crates.io](https://img.shields.io/crates/v/cala.svg)](https://crates.io/crates/cala)
//! [![Zulip Chat](https://img.shields.io/badge/zulip-join_chat-darkgreen.svg)](https://cala.zulipchat.com/join/wkdkw53xb5htnchg8kqz0du0/)
//!
//! [About](https://libcala.github.io/cala) |
//! [Source](https://github.com/libcala/cala) |
//! [Changelog](https://libcala.github.io/cala/changelog) |
//! [Tutorials](https://libcala.github.io/tutorials) |
//! [Blog](https://libcala.github.io)
//!
//! ### Note
//! Cala is a complete redesign of previous library [ADI]("https://crates.io/crates/adi").  It is still in it's early stages.
//!
//! # About
//! Easily create cross-platform applications.  Some common tasks are not easily portable across different platforms, and this crate hopes to fix that.  That way you don't have to worry about how to port your GUI, audio, or bluetooth interface, etc. and can get straight to building your application's content!
//!
//! Cala is a platform-agnostic system interface for hardware IO.  This means that eventually, Cala should support all of the different hardware that's connected to your computer.  Cala is designed so that it talks to the operating system to interface with the hardware, so no special permissions are needed for your application.  Here's a list of all of the targeted platforms (**bold** means a port has been made, *italic* means the feature doesn't work on the platform):
//!
//! - **Linux**
//! - **MacOS** - missing [*audio*](https://github.com/libcala/cala/issues/5), [*controller*](https://github.com/libcala/cala/issues/7), [*graphics*](https://github.com/libcala/cala/issues/9)
//! - **Windows** - missing [*audio*](https://github.com/libcala/cala/issues/4), [*controller*](https://github.com/libcala/cala/issues/6), [*graphics*](https://github.com/libcala/cala/issues/8)
//! - **Web (WASM)** - missing audio, controller, graphics, files
//! - Redox
//! - Android
//! - iOS
//! - Nintendo Switch
//! - XBox
//! - PlayStation
//! - FreeBSD
//! - Maybe FreeDOS for fun 😉️
//! - Others not on this list that you will make a pull request for adding them
//!
//! # Motivation & Naming
//! The aim is to create a newer, better GTK + SDL in Rust!  Why GTK + SDL?  Because a lot of programs need to depend on both anyway (like [totem](https://en.wikipedia.org/wiki/Totem_Video_Player)), and they do a lot of the same things; Usually one library does each specific task better than the other.  The goal of this library is to provide the common ground for video games and general GUI applications together.  The name cala is derived from the fungus known as calafate rust.
//!
//! # Getting Started
//! Each hardware interface can be enabled with a feature.  For example, If you
//! want to depend on the `audio` feature and the `clock`
//! feature, you might put this in your `Cargo.toml`:
//!
//! <!--
//! ```toml
//! [dependencies.cala]
//! version = "0.8"
//! features = ["audio", "clock"]
//! ```
//! -->
//!
//! <p style="width:100%"><pre lang="toml"><code><span style="color:#FFF;font-weight:bold;">[dependencies.cala]</span>
//! <span style="color:#0F0;font-weight:bold;">version</span> = <span style="color:#0F0">"0.8"</span>
//! <span style="color:#0F0;font-weight:bold;">features</span> = [<span style="color:#0F0">"audio"</span>, <span style="color:#0F0">"clock"</span>]</code></pre></p>
//!
//! There is a module for each feature (feature and module names match).  Module documentation may include simple tutorials.  More in depth tutorials may be
//! found [here](https://libcala.github.io/tutorials).

#![warn(missing_docs)]
#![doc(
    html_logo_url = "https://libcala.github.io/logo.svg",
    html_favicon_url = "https://libcala.github.io/icon.svg"
)]

mod run;

#[cfg(feature = "timer")]
mod timer;

#[cfg(feature = "journal")]
pub mod journal;

#[cfg(feature = "user")]
pub mod user {
    //! API for getting user information.  Enable with the `user` feature.
    //!
    //! # Usage
    //! ```rust
    //! // Set the home loop to `run()`.
    //! cala::init!(run, ());
    //!
    //! // Function that runs while your app runs.
    //! pub fn run(_: &mut ()) -> cala::Loop<()> {
    //!     // Print out the user's information.
    //!     println!("{}", cala::user());
    //!     // Exit.
    //!     cala::Exit
    //! }
    //! ```

    include!("internal/whoami.rs");
}

#[cfg(feature = "controller")]
pub mod controller {
    //! API for getting joystick / controller / gamepad input.  Enable with the
    //! `controller` feature.
    //!
    //! # Usage
    //! ```rust
    //! // Set the home loop to `run()`.
    //! cala::init!(run, ());
    //!
    //! // Function that runs while your app runs.
    //! pub fn run(_: &mut ()) -> cala::Loop<()> {
    //!     let layout = cala::ControllerLayout::new().joy(false).lrt(false).abxy(false);
    //!
    //!     // Iterate through all of the controllers.
    //!     'a: for (id, state) in cala::controllers(&layout) {
    //!         println!("{}: {:?}", id, state.get());
    //!     }
    //!     std::thread::sleep(std::time::Duration::from_millis(16));
    //!     // Exit.
    //!     cala::Continue
    //! }
    //! ```

    include!("internal/stick.rs");
}

#[cfg(feature = "audio")]
pub mod audio {
    //! API for recording / playing audio.  Enable with the `audio` feature.
    //!
    //! # Usage
    //! The following example shows how to play audio as it's being recorded.  Headphones
    //! recommended.
    //!
    //! ```rust
    //! use std::collections::VecDeque;
    //!
    //! // The program data context.
    //! struct Data {
    //!     buffer: VecDeque<(i16, i16)>,
    //! }
    //!
    //! // Set the home loop to `run()`.
    //! cala::init!(run, Data {
    //!     buffer: VecDeque::new(),
    //! });
    //!
    //! fn run(data: &mut Data) -> cala::Loop<Data> {
    //!     // Record some sound.
    //!     cala::record(&mut |_whichmic, l, r| {
    //!         data.buffer.push_back((l, r));
    //!     });
    //!
    //!     // Play that sound.
    //!     cala::play(&mut || {
    //!         if let Some((lsample, rsample)) = data.buffer.pop_front() {
    //!             cala::AudioSample::stereo(lsample, rsample)
    //!         } else {
    //!             // Play silence if not enough has been recorded yet.
    //!             cala::AudioSample::stereo(0, 0)
    //!         }
    //!     });
    //!
    //!     cala::Continue
    //! }
    //! ```

    include!("internal/wavy.rs");
}

#[cfg(feature = "files")]
pub mod files {
    //! API for loading & saving files.  Enable with the `files` feature.
    //!
    //! # Usage
    //! ```rust
    //! // TODO
    //! ```

    include!("internal/stronghold.rs");
}

#[cfg(feature = "graphics")]
mod icons;

#[cfg(feature = "graphics")]
#[macro_use]
pub mod graphics {
    //! API for rendering graphics.  Enable with the `graphics` feature.
    //!
    //! # Getting Started
    //! This API is designed to be high-level without sacrificing optimization.
    //! Graphics are complicated though, so before you start, a few things need
    //! to be defined.
    //!
    //! ## Shader
    //! A Shader is a program that runs on the GPU for the purpose of drawing
    //! Shapes.  When you make your program, start by creating a shader.
    //! Shaders are built at compile time, so you'll need to make a build.rs and
    //! depend on the [`res`](https://crates.io/crates/res) crate.  Calling
    //! `generate()` in your build.rs will generate your shaders.
    //!
    //! ## Shape
    //! A shape is a collection of vertices that when connected make a 2D or 3D
    //! shape.  Shapes can only be used with one Shader because they may have
    //! shader-specific additional information attached to them like color or
    //! graphic coordinates.
    //!
    //! ## Instance
    //! Shapes themselves can't be drawn, first you must make an Instance of the
    //! Shape.  Instances can have position attached to them, and/or rotation
    //! and size.
    //!
    //! # Example
    //! ```rust
    //! // TODO
    //! ```

    include!("internal/barg.rs");

    pub use crate::timer::*;
}

#[cfg(feature = "clock")]
pub mod clock;

// Export all types to root.
pub use run::Loop;

#[cfg(feature = "user")]
#[doc(hidden)]
pub use user::*;

#[cfg(feature = "controller")]
#[doc(hidden)]
pub use controller::*;

#[cfg(feature = "audio")]
#[doc(hidden)]
pub use audio::*;

#[cfg(feature = "graphics")]
#[doc(hidden)]
pub use graphics::*;

#[cfg(feature = "clock")]
#[doc(hidden)]
pub use clock::*;

#[doc(hidden)]
pub use internal::start;
#[doc(hidden)]
pub use run::Loop::*;

pub use internal::delta;

//mod audio;
// mod dive;
mod internal;
// mod iolock;

// Others....
//pub use audio::AudioSample;

/// Define the entry point for your program.
///
/// Note that not only is the function an entry point, but also a loop.  Usually you'll
/// want to do your initialization in a block of code for the second parameter.  You can
/// also do additional intialization inside of the initial loop, and then at the end of
/// the function switch to your main loop.
///
/// See [`Loop`](enum.Loop.html) for more details.
///
/// # Usage
/// ```rust
/// // Set the home loop to `run()`.
/// cala::init!(run, ());
///
/// // Function that runs while your app runs.
/// pub fn run(_: &mut ()) -> cala::Loop<()> {
///     // Print out the user's information.
///     println!("{}", cala::user());
///     // Exit.
///     cala::Exit
/// }
/// ```
#[macro_export]
macro_rules! init {
    ($home_loop: expr, $init_data: expr) => {
        fn main() {
            let mut window_title = String::new();
            let mut cap = true;

            let fallback = env!("CARGO_PKG_NAME");

            for c in fallback.chars() {
                match c {
                    '.' | '-' | '_' => {
                        window_title.push(' ');
                        cap = true;
                    }
                    a => {
                        if cap {
                            cap = false;
                            for i in a.to_uppercase() {
                                window_title.push(i);
                            }
                        } else {
                            window_title.push(a);
                        }
                    }
                }
            }

            cala::start(window_title.as_str(), $home_loop, &|| $init_data);
        }
    };
}
