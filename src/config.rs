use serde::de::{Deserialize, Deserializer};
use std::fs;
use std::u32;
use url::Url;

#[derive(Debug, Serialize)]
pub enum Benchmark {
    IO,
    XPU,
    Disabled,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cfg {
    #[serde(with = "url_serde")]
    pub url: Url,

    #[serde(default = "default_timeout")]
    pub timeout: u64,

    #[serde(default = "default_console_log_level")]
    pub console_log_level: String,

    #[serde(default = "default_logfile_log_level")]
    pub logfile_log_level: String,

    #[serde(default = "default_logfile_max_count")]
    pub logfile_max_count: u32,

    #[serde(default = "default_logfile_max_size")]
    pub logfile_max_size: u64,

    #[serde(default = "default_console_log_pattern")]
    pub console_log_pattern: String,

    #[serde(default = "default_logfile_log_pattern")]
    pub logfile_log_pattern: String,

    #[serde(default = "default_debug")]
    pub debug: bool,

    pub benchmark_only: Option<Benchmark>,
}

impl<'de> Deserialize<'de> for Benchmark {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str().to_lowercase().as_ref() {
            "i/o" => Benchmark::IO,
            "xpu" => Benchmark::XPU,
            _ => Benchmark::Disabled,
        })
    }
}


fn default_timeout() -> u64 {
    5000
}

fn default_console_log_level() -> String {
    "Info".to_owned()
}

fn default_logfile_log_level() -> String {
    "Warn".to_owned()
}

fn default_logfile_max_count() -> u32 {
    10
}

fn default_logfile_max_size() -> u64 {
    20
}

fn default_console_log_pattern() -> String {
    "\r{d(%H:%M:%S.%3f%z)} [{h({l}):<5}] [{T}] [{t}] - {M}:{m}{n}".to_owned()
}

fn default_logfile_log_pattern() -> String {
    "\r{d(%Y-%m-%dT%H:%M:%S.%3f%z)} [{h({l}):<5}] [{T}] [{f}:{L}] [{t}] - {M}:{m}{n}".to_owned()
}

fn default_debug() -> bool {
    true
}

pub fn load_cfg(config: &str) -> Cfg {
    let cfg_str = fs::read_to_string(config).expect("failed to open config");
    let cfg: Cfg = serde_yaml::from_str(&cfg_str).expect("failed to parse config");
    return cfg;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_cfg() {
        let cfg = load_cfg("config.yaml");
        assert_eq!(cfg.timeout, 5000);
        let mut pb = PathBuf::new();
        pb.push("test_data");
        assert_eq!(cfg.plot_dirs, vec![pb]);
    }
}