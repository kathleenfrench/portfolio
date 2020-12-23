use std::time::Duration;

/// how often heartbeat pings are sent
pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// how long before lack of client response causes a timeout
pub const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);
