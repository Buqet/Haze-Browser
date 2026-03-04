//! Haze Browser - A modern web browser built on Servo
//!
//! This is the main entry point for the Haze browser application.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use env_logger::Env;
use log::{error, info};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{Key, NamedKey, ModifiersState},
    window::{Window, WindowId},
};

mod browser;
mod config;
mod ui;
mod window;

use browser::Browser;
use config::CliArgs;
use ui::{BrowserAction, BrowserUi, WinitModifiers};

/// Application state
struct App {
    window_id: Option<WindowId>,
    browser: Option<Browser>,
    ui: BrowserUi,
    modifiers: ModifiersState,
    control_flow: ControlFlow,
    cli_args: CliArgs,
}

impl App {
    fn new() -> Self {
        App {
            window_id: None,
            browser: None,
            ui: BrowserUi::new(),
            modifiers: ModifiersState::default(),
            control_flow: ControlFlow::Poll,
            cli_args: CliArgs::parse(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Prevent multiple windows from being created
        if self.window_id.is_some() || self.browser.is_some() {
            return;
        }

        info!("Starting Haze Browser v{}", haze::VERSION);
        info!("Initializing Servo engine...");

        // Initialize Servo crypto
        servoshell::init_crypto();

        // Create the main window
        let window_attrs = Window::default_attributes()
            .with_title(format!("Haze Browser v{}", haze::VERSION))
            .with_inner_size(LogicalSize::new(1280, 800))
            .with_min_inner_size(LogicalSize::new(400, 300));

        match event_loop.create_window(window_attrs) {
            Ok(window) => {
                self.window_id = Some(window.id());

                // Initialize browser with Servo
                self.browser = Some(Browser::new(window));

                // Navigate to initial URL if provided
                if let Some(ref url) = self.cli_args.url {
                    if let Some(ref mut browser) = self.browser {
                        browser.navigate(url.clone());
                        self.ui.update_address_bar(Some(url));
                    }
                } else {
                    // Navigate to home page
                    if let Some(ref mut browser) = self.browser {
                        let home_page = browser.config.home_page.clone();
                        browser.navigate(home_page.clone());
                        self.ui.update_address_bar(Some(&home_page));
                    }
                }

                info!("Window created successfully");
            }
            Err(e) => {
                error!("Failed to create window: {}", e);
                event_loop.exit();
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        // Only handle events for our window
        if Some(window_id) != self.window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                info!("Close requested, shutting down...");
                event_loop.exit();
            }

            WindowEvent::ModifiersChanged(modifiers) => {
                self.modifiers = modifiers.state();
            }

            WindowEvent::KeyboardInput { event, .. } => {
                if event.state.is_pressed() {
                    // Convert to our modifier type
                    let winit_mods = WinitModifiers {
                        ctrl: self.modifiers.control_key(),
                        alt: self.modifiers.alt_key(),
                        shift: self.modifiers.shift_key(),
                        logo: self.modifiers.super_key(),
                    };

                    // Get the key
                    let key = match event.logical_key {
                        Key::Character(ref c) => c.to_lowercase(),
                        Key::Named(named) => {
                            let name = match named {
                                NamedKey::ArrowLeft => "left",
                                NamedKey::ArrowRight => "right",
                                NamedKey::ArrowUp => "up",
                                NamedKey::ArrowDown => "down",
                                NamedKey::F11 => "f11",
                                NamedKey::F5 => "f5",
                                NamedKey::Enter => "enter",
                                NamedKey::Escape => "escape",
                                NamedKey::Space => "space",
                                NamedKey::Tab => "tab",
                                NamedKey::Backspace => "backspace",
                                NamedKey::Delete => "delete",
                                NamedKey::Home => "home",
                                NamedKey::End => "end",
                                NamedKey::PageUp => "pageup",
                                NamedKey::PageDown => "pagedown",
                                NamedKey::Insert => "insert",
                                _ => "",
                            };
                            name.to_string()
                        }
                        _ => String::new(),
                    };

                    // Check for shortcuts
                    if let Some(action) = self.ui.handle_shortcut(&key, winit_mods) {
                        self.handle_action(action, event_loop);
                    }

                    // Handle address bar input
                    if self.ui.address_bar_focused && !self.modifiers.control_key() && !self.modifiers.alt_key() {
                        // TODO: Handle text input for address bar
                    }
                }
            }

            WindowEvent::RedrawRequested => {
                if let Some(browser) = &self.browser {
                    self.ui.render(browser);
                }
            }

            WindowEvent::Focused(focused) => {
                if let Some(ref browser) = self.browser {
                    browser.window.set_focused(focused);
                }
                if !focused {
                    self.ui.address_bar_focused = false;
                }
            }

            WindowEvent::Resized(size) => {
                if let Some(ref mut browser) = self.browser {
                    // TODO: Resize WebView to match window
                    info!("Window resized to: {:?}", size);
                }
            }

            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        _event_loop.set_control_flow(self.control_flow);
    }
}

impl App {
    fn handle_action(&mut self, action: BrowserAction, _event_loop: &ActiveEventLoop) {
        let Some(ref mut browser) = self.browser else {
            return;
        };

        match action {
            BrowserAction::FocusAddressBar => {
                self.ui.address_bar_focused = true;
                if let Some(url) = browser.current_url() {
                    self.ui.address_bar = url.to_string();
                }
            }
            BrowserAction::Reload => {
                browser.reload();
            }
            BrowserAction::CloseTab => {
                // TODO: Implement tab closing
                _event_loop.exit();
            }
            BrowserAction::NewTab => {
                // TODO: Implement new tab
                info!("New tab requested");
            }
            BrowserAction::GoBack => {
                browser.go_back();
            }
            BrowserAction::GoForward => {
                browser.go_forward();
            }
            BrowserAction::Stop => {
                browser.stop();
            }
            BrowserAction::ZoomReset => {
                self.ui.zoom_reset();
            }
            BrowserAction::ZoomIn => {
                self.ui.zoom_in();
            }
            BrowserAction::ZoomOut => {
                self.ui.zoom_out();
            }
            BrowserAction::ToggleFullscreen => {
                let is_fullscreen = browser.window.winit_window().fullscreen().is_some();
                browser.window.winit_window().set_fullscreen(if is_fullscreen { None } else { Some(winit::window::Fullscreen::Borderless(None)) });
            }
            BrowserAction::Navigate(url) => {
                browser.navigate(url);
            }
        }
    }
}

fn main() {
    // Initialize logging
    let log_level = if CliArgs::parse().verbose { "debug" } else { "info" };
    env_logger::Builder::from_env(Env::default().default_filter_or(log_level)).init();

    info!("Haze Browser starting...");
    info!("Version: {}", haze::VERSION);

    // Create event loop
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);

    // Create app state
    let mut app = App::new();

    // Run the event loop
    match event_loop.run_app(&mut app) {
        Ok(()) => info!("Browser exited normally"),
        Err(e) => error!("Browser error: {}", e),
    }

    // Cleanup Servo
    servoshell::platform::deinit(true);
    info!("Haze Browser shutdown complete");
}
