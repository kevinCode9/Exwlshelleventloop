use std::{
    fmt,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use exwlshellev::WindowWrapper;
use iced_core::clipboard::{Content, Error, Kind};

static DISABLED: AtomicBool = AtomicBool::new(false);

pub(crate) fn set_disabled() {
    DISABLED.store(true, Ordering::Relaxed);
}

pub(crate) fn is_disabled() -> bool {
    DISABLED.load(Ordering::Relaxed)
}

pub struct ExwlShellClipboard {
    state: State,
}

enum State {
    Connected(window_clipboard::Clipboard),
    Unavailable,
}

impl ExwlShellClipboard {
    /// Creates a new [`Clipboard`] for the given window.
    pub fn connect(window: &WindowWrapper) -> Self {
        #[allow(unsafe_code)]
        let state = unsafe { window_clipboard::Clipboard::connect(window) }
            .ok()
            .map(State::Connected)
            .unwrap_or(State::Unavailable);

        Self { state }
    }

    /// Creates a new [`Clipboard`] that isn't associated with a window.
    /// This clipboard will never contain a copied value.
    #[allow(unused)]
    pub fn unconnected() -> Self {
        Self {
            state: State::Unavailable,
        }
    }

    /// Reads the current content of the [`Clipboard`] as text.
    pub fn read(&self, kind: Kind) -> Result<Content, Error> {
        match &self.state {
            State::Connected(clipboard) => match kind {
                Kind::Text => clipboard.read().map(Content::Text).map_err(map_error),
                _ => Err(Error::ContentNotAvailable),
            },
            State::Unavailable => Err(Error::ClipboardUnavailable),
        }
    }

    /// Writes the given text contents to the [`Clipboard`].
    pub fn write(&mut self, content: Content) -> Result<(), Error> {
        match &mut self.state {
            State::Connected(clipboard) => match content {
                Content::Text(text) => clipboard.write(text).map_err(map_error),
                _ => Err(Error::ConversionFailure),
            },
            State::Unavailable => Err(Error::ClipboardUnavailable),
        }
    }
}

fn map_error(error: impl fmt::Display) -> Error {
    Error::Unknown {
        description: Arc::new(error.to_string()),
    }
}
