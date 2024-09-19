use std::{env, fs, sync::OnceLock};

use serde::Deserialize;

use crate::constants::DEFAULT_SETTINGS_PATH;

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub exchange: String,
    
    pub udp_auto_switch: bool,
    pub udp_switch_timeout: usize,
    pub udp_local_ip: String,
    pub primary_mcast_ip: String,
    pub primary_mcast_port: usize,
    pub secondary_mcast_ip: String,
    pub secondary_mcast_port: usize,

    pub thread_count: usize,

    pub kafka_partition_no: usize,
    pub kafka_brokers: usize,
    pub kafka_topic_name: String,
    pub kafka_config_path: String,
}

pub static  SETTINGS: OnceLock<Settings> = OnceLock::new();

pub fn init() {
    let args = env::args().collect::<Vec<String>>();

    // Get settings path
    let default_path = DEFAULT_SETTINGS_PATH.to_string();
    let settings_path = args.get(1).unwrap_or(&default_path);

    // Read settings to string
    let settings = fs::read_to_string(settings_path).expect("Cannot find config file");

    // Create struct from settings file
    let settings = serde_json::from_str::<Settings>(&settings).expect("Unable to parse settings file");

    // Initialize settings
    SETTINGS.get_or_init(||{
        settings
    });
}

pub fn get() -> &'static Settings {
    // Initialize settings
    SETTINGS.get().unwrap()
}