use serde::Serialize;
use std::fmt;
use std::process::Command;
use std::time::Instant;

#[derive(Debug, Serialize, Clone, Copy)]
pub enum PingStatus {
    #[serde(rename = "REACHABLE")]
    Reachable,

    #[serde(rename = "TIMEOUT")]
    Timeout,

    #[serde(rename = "ERROR")]
    Error,
}

impl fmt::Display for PingStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PingStatus::Reachable => write!(formatter, "REACHABLE"),
            PingStatus::Timeout => write!(formatter, "TIMEOUT"),
            PingStatus::Error => write!(formatter, "ERROR"),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct PingResult {
    pub status: PingStatus,
    pub response_time_ms: Option<u64>,
}

pub fn ping_host(address: &str, timeout_ms: u64) -> PingResult {
    let start_time = Instant::now();

    let timeout_seconds = milliseconds_to_ping_timeout_seconds(timeout_ms);

    let output = Command::new("ping")
        .arg("-c")
        .arg("1")
        .arg("-W")
        .arg(timeout_seconds.to_string())
        .arg(address)
        .output();

    match output {
        Ok(output) => {
            let elapsed_ms = start_time.elapsed().as_millis() as u64;

            if output.status.success() {
                PingResult {
                    status: PingStatus::Reachable,
                    response_time_ms: Some(elapsed_ms),
                }
            } else {
                PingResult {
                    status: PingStatus::Timeout,
                    response_time_ms: Some(elapsed_ms),
                }
            }
        }
        Err(_) => PingResult {
            status: PingStatus::Error,
            response_time_ms: None,
        },
    }
}

fn milliseconds_to_ping_timeout_seconds(timeout_ms: u64) -> u64 {
    let seconds = timeout_ms.div_ceil(1000);

    if seconds == 0 { 1 } else { seconds }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_status_display_should_return_correct_text() {
        assert_eq!(PingStatus::Reachable.to_string(), "REACHABLE");
        assert_eq!(PingStatus::Timeout.to_string(), "TIMEOUT");
        assert_eq!(PingStatus::Error.to_string(), "ERROR");
    }

    #[test]
    fn timeout_conversion_should_round_up_to_seconds() {
        assert_eq!(milliseconds_to_ping_timeout_seconds(1), 1);
        assert_eq!(milliseconds_to_ping_timeout_seconds(999), 1);
        assert_eq!(milliseconds_to_ping_timeout_seconds(1000), 1);
        assert_eq!(milliseconds_to_ping_timeout_seconds(1001), 2);
        assert_eq!(milliseconds_to_ping_timeout_seconds(2500), 3);
    }
}
