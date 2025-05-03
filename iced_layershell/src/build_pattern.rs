//! The build_pattern allow you to create application just with callback functions.
//! Similar with the one of origin iced.

mod application;
mod daemon;
use std::borrow::Cow;

use iced::{Font, Pixels};

use crate::settings::{LayerShellSettings, VirtualKeyboardSettings};

/// The renderer of some Program.
pub trait Renderer: iced_core::text::Renderer + iced_graphics::compositor::Default {}

impl<T> Renderer for T where T: iced_core::text::Renderer + iced_graphics::compositor::Default {}

/// MainSettings for iced_layershell
/// different from [`crate::Settings`], it does not contain the field of flags
#[derive(Debug)]
pub struct Settings {
    /// The identifier of the application.
    ///
    /// If provided, this identifier may be used to identify the application or
    /// communicate with it through the windowing system.
    pub id: Option<String>,

    /// settings for layer shell
    pub layer_settings: LayerShellSettings,
    /// The data needed to initialize an Application
    ///
    /// The fonts to load on boot.
    pub fonts: Vec<Cow<'static, [u8]>>,

    /// The default [`Font`] to be used.
    ///
    /// By default, it uses [`Family::SansSerif`](iced::font::Family::SansSerif).
    pub default_font: Font,

    /// The text size that will be used by default.
    ///
    /// The default value is `16.0`.
    pub default_text_size: Pixels,

    /// If set to true, the renderer will try to perform antialiasing for some
    /// primitives.
    ///
    /// Enabling it can produce a smoother result in some widgets, like the
    /// `Canvas`, at a performance cost.
    ///
    /// By default, it is disabled.
    ///
    pub antialiasing: bool,

    pub virtual_keyboard_support: Option<VirtualKeyboardSettings>,
}
impl Default for Settings {
    fn default() -> Self {
        Settings {
            id: None,
            fonts: Vec::new(),
            layer_settings: LayerShellSettings::default(),
            default_font: Font::default(),
            default_text_size: Pixels(16.0),
            antialiasing: false,
            virtual_keyboard_support: None,
        }
    }
}

#[doc = include_str!("./build_pattern/application.md")]
pub use application::application;

#[doc = include_str!("./build_pattern/daemon.md")]
pub use daemon::daemon;
