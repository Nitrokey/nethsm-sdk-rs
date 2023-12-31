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
pub enum EncryptMode {
    #[serde(rename = "AES_CBC")]
    AesCbc,
}

impl ToString for EncryptMode {
    fn to_string(&self) -> String {
        match self {
            Self::AesCbc => String::from("AES_CBC"),
        }
    }
}

impl Default for EncryptMode {
    fn default() -> EncryptMode {
        Self::AesCbc
    }
}
