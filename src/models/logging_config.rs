/*
 * NetHSM
 *
 * All endpoints expect exactly the specified JSON. Additional properties will cause a Bad Request Error (400). All HTTP errors contain a JSON structure with an explanation of type string. All [base64](https://tools.ietf.org/html/rfc4648#section-4) encoded values are Big Endian.
 *
 * The version of the OpenAPI document: v1
 * Contact: Nitrokey <info@nitrokey.com>
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoggingConfig {
    #[serde(rename = "ipAddress")]
    pub ip_address: String,
    #[serde(rename = "port")]
    pub port: i32,
    #[serde(rename = "logLevel")]
    pub log_level: crate::models::LogLevel,
}

impl LoggingConfig {
    pub fn new(ip_address: String, port: i32, log_level: crate::models::LogLevel) -> LoggingConfig {
        LoggingConfig {
            ip_address,
            port,
            log_level,
        }
    }
}
