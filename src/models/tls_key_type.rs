/*
 * NetHSM
 *
 * All endpoints expect exactly the specified JSON. Additional properties will cause a Bad Request Error (400). All HTTP errors contain a JSON structure with an explanation of type string. All [base64](https://tools.ietf.org/html/rfc4648#section-4) encoded values are Big Endian.
 *
 * The version of the OpenAPI document: v1
 * Contact: Nitrokey <info@nitrokey.com>
 * Generated by: https://openapi-generator.tech
 */

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TlsKeyType {
    #[serde(rename = "RSA")]
    Rsa,
    #[serde(rename = "Curve25519")]
    Curve25519,
    #[serde(rename = "EC_P224")]
    EcP224,
    #[serde(rename = "EC_P256")]
    EcP256,
    #[serde(rename = "EC_P384")]
    EcP384,
    #[serde(rename = "EC_P521")]
    EcP521,
}

impl ToString for TlsKeyType {
    fn to_string(&self) -> String {
        match self {
            Self::Rsa => String::from("RSA"),
            Self::Curve25519 => String::from("Curve25519"),
            Self::EcP224 => String::from("EC_P224"),
            Self::EcP256 => String::from("EC_P256"),
            Self::EcP384 => String::from("EC_P384"),
            Self::EcP521 => String::from("EC_P521"),
        }
    }
}

impl Default for TlsKeyType {
    fn default() -> TlsKeyType {
        Self::Rsa
    }
}
