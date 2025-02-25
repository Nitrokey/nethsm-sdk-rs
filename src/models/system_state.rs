/*
 * NetHSM
 *
 * All endpoints expect exactly the specified JSON. Additional properties will cause a Bad Request Error (400). All HTTP errors contain a JSON structure with an explanation of type string. All [base64](https://tools.ietf.org/html/rfc4648#section-4) encoded values are Big Endian.
 *
 * The version of the OpenAPI document: v1
 * Contact: Nitrokey <info@nitrokey.com>
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SystemState {
    #[serde(rename = "Unprovisioned")]
    Unprovisioned,
    #[serde(rename = "Locked")]
    Locked,
    #[serde(rename = "Operational")]
    Operational,
}

impl std::fmt::Display for SystemState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Unprovisioned => "Unprovisioned",
                Self::Locked => "Locked",
                Self::Operational => "Operational",
            }
        )
    }
}

impl Default for SystemState {
    fn default() -> SystemState {
        Self::Unprovisioned
    }
}
