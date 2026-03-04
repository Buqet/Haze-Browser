//! UI components for the browser chrome

use crate::browser::Browser;
use log::info;

/// Browser UI state
pub struct BrowserUi {
    /// Address bar text
    pub address_bar: String,
    /// Whether the address bar is focused
    pub address_bar_focused: bool,
    /// Zoom level (1.0 = 100%)
    pub zoom_level: f32,
    /// Status bar message
    pub status_message: Option<String>,
}

impl Default for BrowserUi {
    fn default() -> Self {
        BrowserUi {
            address_bar: String::new(),
            address_bar_focused: false,
            zoom_level: 1.0,
            status_message: None,
        }
    }
}

impl BrowserUi {
    /// Create new UI state
    pub fn new() -> Self {
        Self::default()
    }

    /// Update address bar with current URL
    pub fn update_address_bar(&mut self, url: Option<&str>) {
        self.address_bar = url.unwrap_or("").to_string();
    }

    /// Set status message
    pub fn set_status(&mut self, message: Option<String>) {
        self.status_message = message;
    }

    /// Zoom in
    pub fn zoom_in(&mut self) {
        self.zoom_level = (self.zoom_level + 0.1).min(5.0);
        info!("Zoom level: {:.0}%", self.zoom_level * 100.0);
    }

    /// Zoom out
    pub fn zoom_out(&mut self) {
        self.zoom_level = (self.zoom_level - 0.1).max(0.5);
        info!("Zoom level: {:.0}%", self.zoom_level * 100.0);
    }

    /// Reset zoom
    pub fn zoom_reset(&mut self) {
        self.zoom_level = 1.0;
        info!("Zoom level reset");
    }

    /// Render the UI
    pub fn render(&self, _browser: &Browser) {
        // TODO: Implement actual UI rendering
        // For now, this is a placeholder
    }

    /// Handle keyboard shortcuts
    pub fn handle_shortcut(&mut self, key: &str, modifiers: WinitModifiers) -> Option<BrowserAction> {
        // Ctrl/Cmd + L - Focus address bar
        if modifiers.ctrl && key == "l" {
            self.address_bar_focused = true;
            return Some(BrowserAction::FocusAddressBar);
        }

        // Ctrl/Cmd + R - Reload
        if modifiers.ctrl && key == "r" {
            return Some(BrowserAction::Reload);
        }

        // Ctrl/Cmd + W - Close tab
        if modifiers.ctrl && key == "w" {
            return Some(BrowserAction::CloseTab);
        }

        // Ctrl/Cmd + T - New tab
        if modifiers.ctrl && key == "t" {
            return Some(BrowserAction::NewTab);
        }

        // Alt + Left - Back
        if modifiers.alt && key == "left" {
            return Some(BrowserAction::GoBack);
        }

        // Alt + Right - Forward
        if modifiers.alt && key == "right" {
            return Some(BrowserAction::GoForward);
        }

        // Ctrl/Cmd + 0 - Reset zoom
        if modifiers.ctrl && key == "0" {
            self.zoom_reset();
            return Some(BrowserAction::ZoomReset);
        }

        // Ctrl/Cmd + Plus - Zoom in
        if modifiers.ctrl && (key == "=" || key == "+") {
            self.zoom_in();
            return Some(BrowserAction::ZoomIn);
        }

        // Ctrl/Cmd + Minus - Zoom out
        if modifiers.ctrl && key == "-" {
            self.zoom_out();
            return Some(BrowserAction::ZoomOut);
        }

        // F11 - Fullscreen
        if key == "f11" {
            return Some(BrowserAction::ToggleFullscreen);
        }

        // F5 - Reload
        if key == "f5" {
            return Some(BrowserAction::Reload);
        }

        // Escape - Stop loading
        if key == "escape" {
            return Some(BrowserAction::Stop);
        }

        None
    }
}

/// Keyboard modifiers helper
#[derive(Debug, Clone, Copy, Default)]
pub struct WinitModifiers {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub logo: bool,
}

impl WinitModifiers {
    pub fn ctrl(&self) -> bool {
        self.ctrl || self.logo // Cmd on macOS
    }

    pub fn alt(&self) -> bool {
        self.alt
    }

    pub fn shift(&self) -> bool {
        self.shift
    }
}

/// Actions that can be triggered from UI
#[derive(Debug, Clone)]
pub enum BrowserAction {
    FocusAddressBar,
    Reload,
    CloseTab,
    NewTab,
    GoBack,
    GoForward,
    Stop,
    ZoomReset,
    ZoomIn,
    ZoomOut,
    ToggleFullscreen,
    Navigate(String),
}
