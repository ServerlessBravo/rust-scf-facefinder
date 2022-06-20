use std::env;

use once_cell::sync::Lazy;

pub static CONFIG: Lazy<Config> = Lazy::new(Config::default);

/// 从环境变量中读取SCF配置
#[derive(Debug)]
pub struct Config {
    pub scf_host: String,
    pub scf_port: String,
    pub func_name: String,
    pub ready_url: String,
    pub event_url: String,
    pub response_url: String,
    pub error_url: String,
}

impl Default for Config {
    fn default() -> Self {
        let scf_host = env::var("SCF_RUNTIME_API").unwrap_or_default();
        let scf_port = env::var("SCF_RUNTIME_API_PORT").unwrap_or_default();
        let func_name = env::var("_HANDLER").unwrap_or_default();

        Self {
            ready_url: format!("http://{scf_host}:{scf_port}/runtime/init/ready"),
            event_url: format!("http://{scf_host}:{scf_port}/runtime/invocation/next"),
            response_url: format!("http://{scf_host}:{scf_port}/runtime/invocation/response"),
            error_url: format!("http://{scf_host}:{scf_port}/runtime/invocation/error"),
            scf_host,
            scf_port,
            func_name,
        }
    }
}
