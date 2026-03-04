//! Browser configuration and settings

use std::path::PathBuf;

/// Browser configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// Home page URL
    pub home_page: String,
    /// Search engine URL (with {query} placeholder)
    pub search_engine: String,
    /// User agent string
    pub user_agent: String,
    /// Enable hardware acceleration
    pub hardware_acceleration: bool,
    /// Enable WebGL
    pub webgl_enabled: bool,
    /// Enable WebGPU
    pub webgpu_enabled: bool,
    /// Default downloads directory
    pub download_dir: PathBuf,
    /// Cache directory
    pub cache_dir: PathBuf,
    /// Enable JavaScript
    pub javascript_enabled: bool,
    /// Enable images
    pub images_enabled: bool,
    /// Enable cookies
    pub cookies_enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            home_page: "about:blank".to_string(),
            search_engine: "https://www.google.com/search?q={query}".to_string(),
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Haze/0.1.0 Safari/537.36".to_string(),
            hardware_acceleration: true,
            webgl_enabled: true,
            webgpu_enabled: true,
            javascript_enabled: true,
            images_enabled: true,
            cookies_enabled: true,
            download_dir: dirs::download_dir().unwrap_or_else(|| PathBuf::from("./downloads")),
            cache_dir: dirs::cache_dir()
                .unwrap_or_else(|| PathBuf::from("./cache"))
                .join("haze"),
        }
    }
}

impl Config {
    /// Load configuration from file
    pub fn load() -> Self {
        // TODO: Load from config file (TOML/JSON)
        Config::default()
    }

    /// Save configuration to file
    pub fn save(&self) -> std::io::Result<()> {
        // TODO: Save to config file
        Ok(())
    }

    /// Get the cache directory, creating it if necessary
    pub fn ensure_cache_dir(&self) -> std::io::Result<&PathBuf> {
        std::fs::create_dir_all(&self.cache_dir)?;
        Ok(&self.cache_dir)
    }
}

/// Command line arguments parser
pub struct CliArgs {
    /// URL to open
    pub url: Option<String>,
    /// Headless mode
    pub headless: bool,
    /// Enable verbose logging
    pub verbose: bool,
    /// Disable GPU acceleration
    pub no_gpu: bool,
}

impl Default for CliArgs {
    fn default() -> Self {
        CliArgs {
            url: None,
            headless: false,
            verbose: false,
            no_gpu: false,
        }
    }
}

impl CliArgs {
    /// Parse command line arguments
    pub fn parse() -> Self {
        let args: Vec<String> = std::env::args().collect();
        let mut result = CliArgs::default();

        for arg in args.iter().skip(1) {
            if arg == "--headless" {
                result.headless = true;
            } else if arg == "--verbose" || arg == "-v" {
                result.verbose = true;
            } else if arg == "--no-gpu" {
                result.no_gpu = true;
            } else if !arg.starts_with('-') {
                result.url = Some(arg.clone());
            }
        }

        result
    }
}
