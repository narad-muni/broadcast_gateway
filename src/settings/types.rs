use core::fmt;
use std::str::FromStr;

use bitflags::bitflags;
use serde::{
    de::{self, SeqAccess, Visitor},
    Deserialize, Deserializer,
};

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

    pub output_udp_ip: String,
    pub output_udp_port: usize,

    pub thread_count: usize,

    pub kafka_partition_no: usize,
    pub kafka_brokers: String,
    pub kafka_topic_name: String,
    pub kafka_config_path: String,

    pub output_targets: OutputTargets,
}

bitflags! {
    #[derive(Clone, Debug)]
    pub struct OutputTargets: u8 {
        const UDP = 1;
        const KAFKA = 2;
        const STDOUT = 4;
    }
}

impl<'de> Deserialize<'de> for OutputTargets {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OutputTargetsVisitor;

        impl<'de> Visitor<'de> for OutputTargetsVisitor {
            type Value = OutputTargets;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an array of strings representing log targets")
            }

            // Handle an array of strings
            fn visit_seq<A>(self, mut seq: A) -> Result<OutputTargets, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut flags = OutputTargets::empty();

                // Loop over each string in the array
                while let Some(value) = seq.next_element::<String>()? {
                    let log_target = OutputTargets::from_str(&value).map_err(de::Error::custom)?;

                    // Combine the flags using bitwise OR
                    flags |= log_target;
                }

                Ok(flags)
            }
        }

        deserializer.deserialize_seq(OutputTargetsVisitor)
    }
}

impl FromStr for OutputTargets {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "udp" => Ok(OutputTargets::UDP),
            "kafka" => Ok(OutputTargets::KAFKA),
            "stdout" => Ok(OutputTargets::STDOUT),
            _ => Err("Invalid log target"),
        }
    }
}
