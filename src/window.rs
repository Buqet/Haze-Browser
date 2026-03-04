//! Browser window management

use std::cell::Cell;
use std::rc::Rc;
use winit::window::Window;

/// Browser window
pub struct BrowserWindow {
    /// The winit window
    pub window: Rc<Window>,
    /// Whether the window is focused
    pub focused: Cell<bool>,
    /// Whether the window is minimized
    pub minimized: Cell<bool>,
}

impl BrowserWindow {
    /// Create a new browser window
    pub fn new(window: Window) -> Rc<Self> {
        Rc::new(BrowserWindow {
            window: Rc::new(window),
            focused: Cell::new(true),
            minimized: Cell::new(false),
        })
    }

    /// Get the winit window
    pub fn winit_window(&self) -> &Window {
        &self.window
    }

    /// Set focus state
    pub fn set_focused(&self, focused: bool) {
        self.focused.set(focused);
    }

    /// Set minimized state
    pub fn set_minimized(&self, minimized: bool) {
        self.minimized.set(minimized);
    }

    /// Check if window is focused
    pub fn is_focused(&self) -> bool {
        self.focused.get()
    }

    /// Check if window is minimized
    pub fn is_minimized(&self) -> bool {
        self.minimized.get()
    }
}

/// Window state for tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    /// Window is active and visible
    Active,
    /// Window is minimized
    Minimized,
    /// Window is hidden
    Hidden,
}

impl Default for WindowState {
    fn default() -> Self {
        WindowState::Active
    }
}
