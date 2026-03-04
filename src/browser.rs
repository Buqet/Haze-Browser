//! Browser core with Servo integration

use log::info;
use servo::{ServoBuilder, WebView, WebViewBuilder, WindowRenderingContext};
use std::cell::RefCell;
use std::rc::Rc;
use winit::dpi::PhysicalSize;
use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::window::Window;

use crate::config::Config;
use crate::window::BrowserWindow;

/// Main browser struct integrating Servo WebView
pub struct Browser {
    /// The Servo WebView
    pub webview: Rc<RefCell<WebView>>,
    /// The browser window
    pub window: Rc<BrowserWindow>,
    /// Current URL
    pub current_url: Option<String>,
    /// Navigation history (back)
    pub history_back: Vec<String>,
    /// Navigation history (forward)
    pub history_forward: Vec<String>,
    /// Whether the page is loading
    pub is_loading: bool,
    /// Page title
    pub page_title: Option<String>,
    /// Browser configuration
    pub config: Config,
    /// Servo instance
    pub servo: servo::Servo,
}

impl Browser {
    /// Create a new browser instance with Servo WebView
    pub fn new(window: Window) -> Self {
        info!("Initializing Haze Browser with Servo...");

        let browser_window = BrowserWindow::new(window);

        let size = browser_window.window.inner_size();
        let display_handle = browser_window.window.display_handle().expect("Failed to get display handle");
        let window_handle = browser_window.window.window_handle().expect("Failed to get window handle");

        // Initialize Servo using ServoBuilder
        let servo = ServoBuilder::default().build();
        
        // Create WebView using WebViewBuilder with WindowRenderingContext
        let rendering_context = WindowRenderingContext::new(
            display_handle,
            window_handle,
            PhysicalSize::new(size.width, size.height),
        ).expect("Failed to create WindowRenderingContext");
        
        let webview = Rc::new(RefCell::new(
            WebViewBuilder::new(&servo, Rc::new(rendering_context))
                .build()
        ));

        info!("Servo WebView initialized");

        Browser {
            webview,
            window: browser_window,
            current_url: None,
            history_back: Vec::new(),
            history_forward: Vec::new(),
            is_loading: false,
            page_title: None,
            config: Config::default(),
            servo,
        }
    }

    /// Navigate to a URL
    pub fn navigate(&mut self, url: String) {
        info!("Navigating to: {}", url);

        // Parse and validate URL
        let parsed_url = match self.parse_url(&url) {
            Ok(u) => u,
            Err(e) => {
                log::error!("Failed to parse URL: {}", e);
                return;
            }
        };

        // Add current URL to history
        if let Some(current) = self.current_url.take() {
            self.history_back.push(current);
            self.history_forward.clear();
        }

        self.current_url = Some(parsed_url);
        self.is_loading = true;

        // TODO: Actually navigate using Servo's constellation
        info!("Navigation initiated");
    }

    /// Go back in history
    pub fn go_back(&mut self) {
        if let Some(url) = self.history_back.pop() {
            if let Some(current) = self.current_url.take() {
                self.history_forward.push(current);
            }
            self.current_url = Some(url.clone());
            self.is_loading = true;
            info!("Going back to: {}", url);
        }
    }

    /// Go forward in history
    pub fn go_forward(&mut self) {
        if let Some(url) = self.history_forward.pop() {
            if let Some(current) = self.current_url.take() {
                self.history_back.push(current);
            }
            self.current_url = Some(url.clone());
            self.is_loading = true;
            info!("Going forward to: {}", url);
        }
    }

    /// Reload the current page
    pub fn reload(&mut self) {
        if self.current_url.is_some() {
            self.is_loading = true;
            info!("Reloading current page");
        }
    }

    /// Stop loading
    pub fn stop(&mut self) {
        self.is_loading = false;
        info!("Stopped loading");
    }

    /// Parse URL (handles both full URLs and search queries)
    fn parse_url(&self, input: &str) -> Result<String, String> {
        // Check if it's already a URL
        if input.starts_with("http://") || input.starts_with("https://") || input.starts_with("file://") {
            return Ok(input.to_string());
        }

        // Check if it looks like a domain
        if input.contains('.') && !input.contains(' ') {
            return Ok(format!("https://{}", input));
        }

        // Otherwise, treat as search query
        let encoded = urlencoding::encode(input);
        Ok(format!("https://www.google.com/search?q={}", encoded))
    }

    /// Get the current URL
    pub fn current_url(&self) -> Option<&str> {
        self.current_url.as_deref()
    }

    /// Get the page title
    pub fn page_title(&self) -> Option<&str> {
        self.page_title.as_deref()
    }

    /// Check if can go back
    pub fn can_go_back(&self) -> bool {
        !self.history_back.is_empty()
    }

    /// Check if can go forward
    pub fn can_go_forward(&self) -> bool {
        !self.history_forward.is_empty()
    }

    /// Check if currently loading
    pub fn is_loading(&self) -> bool {
        self.is_loading
    }

    /// Get the browser window
    pub fn window(&self) -> &BrowserWindow {
        &self.window
    }

    /// Get browser configuration
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Set page title (called by WebView delegate)
    pub fn set_page_title(&mut self, title: String) {
        info!("Page title changed: {}", title);
        self.page_title = Some(title);
    }

    /// Set loading state (called by WebView delegate)
    pub fn set_loading(&mut self, loading: bool) {
        self.is_loading = loading;
    }
}

/// Browser events
#[derive(Debug, Clone)]
pub enum BrowserEvent {
    /// Navigate to URL
    Navigate(String),
    /// Go back
    GoBack,
    /// Go forward
    GoForward,
    /// Reload page
    Reload,
    /// Stop loading
    Stop,
    /// New tab requested
    NewTab,
    /// Close tab requested
    CloseTab(usize),
    /// Select tab
    SelectTab(usize),
    /// Page title changed
    TitleChanged(String),
    /// Loading state changed
    LoadingChanged(bool),
    /// URL changed
    UrlChanged(String),
}
