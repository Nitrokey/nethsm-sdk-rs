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
pub struct PrivateKeyPemArguments {
    #[serde(rename = "mechanisms", skip_serializing_if = "Option::is_none")]
    pub mechanisms: Option<Vec<crate::models::KeyMechanism>>,
    #[serde(rename = "restrictions", skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Box<crate::models::KeyRestrictions>>,
}

impl PrivateKeyPemArguments {
    pub fn new() -> PrivateKeyPemArguments {
        PrivateKeyPemArguments {
            mechanisms: None,
            restrictions: None,
        }
    }
}
