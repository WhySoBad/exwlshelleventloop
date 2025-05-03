use std::borrow::Cow;

use iced::{Font, Pixels};

pub use crate::build_pattern::Settings;

#[derive(Debug, Clone)]
pub struct SettingsMain<Flags> {
    /// The identifier of the application.
    ///
    /// If provided, this identifier may be used to identify the application or
    /// communicate with it through the windowing system.
    pub id: Option<String>,

    /// The data needed to initialize an [`MultiApplication`].
    ///
    /// [`MultiApplication`]: crate::MultiApplication
    pub flags: Flags,

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
}

impl<Flags> Default for SettingsMain<Flags>
where
    Flags: Default,
{
    fn default() -> Self {
        SettingsMain {
            id: None,
            flags: Default::default(),
            fonts: Vec::new(),
            default_font: Font::default(),
            default_text_size: Pixels(16.0),
            antialiasing: false,
        }
    }
}
