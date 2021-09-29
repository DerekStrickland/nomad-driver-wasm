use super::fingerprinter::FingerprintError;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
// HealthCheck is used for doing periodic health checks. On a given time
// interval, a health check will be called by the fingerprint manager of the
// node.
trait HealthCheck {
    // Check is used to update properties of the node on the status of the health
    // check
    fn check(
        &self,
        request: HealthCheckRequest,
        response: HealthCheckResponse,
    ) -> Result<Box<HealthCheckResponse>, FingerprintError>;

    // get_health_check_interval is a mechanism for the health checker to indicate that
    // it should be run periodically. The return value is a boolean indicating
    // whether it should be done periodically, and the time interval at which
    // this check should happen.
    fn get_health_check_interval(
        &self,
        request: HealthCheckIntervalRequest,
        response: HealthCheckIntervalResponse,
    ) -> Result<Box<HealthCheckIntervalResponse>, FingerprintError>;
}

// HealthCheckRequest is the request type for a type that fulfils the Health
// Check interface
pub struct HealthCheckRequest {}

// HealthCheckResponse is the response type for a type that fulfills the Health
// Check interface
pub struct HealthCheckResponse {
    // Drivers is a map of driver names to current driver information
    drivers: HashMap<String, DriverInfo>,
}

// HealthCheckIntervalRequest is the request type for a type that fulfils the
// Health Check Interval interface
pub struct HealthCheckIntervalRequest {}

// HealthCheckIntervalResponse is the request type for a type that fulfils the
// Health Check Interval interface
pub struct HealthCheckIntervalResponse {
    eligible: bool,
    period: Duration,
}

// TODO: Figure out serialization issues
// DriverInfo contains the health summary for the current driver.
pub struct DriverInfo {
    attributes: HashMap<String, String>,
    detected: bool,
    healthy: bool,
    health_description: String,
    update_time: SystemTime,
}
