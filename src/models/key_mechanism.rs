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
pub enum KeyMechanism {
    #[serde(rename = "RSA_Decryption_RAW")]
    RsaDecryptionRaw,
    #[serde(rename = "RSA_Decryption_PKCS1")]
    RsaDecryptionPkcs1,
    #[serde(rename = "RSA_Decryption_OAEP_MD5")]
    RsaDecryptionOaepMd5,
    #[serde(rename = "RSA_Decryption_OAEP_SHA1")]
    RsaDecryptionOaepSha1,
    #[serde(rename = "RSA_Decryption_OAEP_SHA224")]
    RsaDecryptionOaepSha224,
    #[serde(rename = "RSA_Decryption_OAEP_SHA256")]
    RsaDecryptionOaepSha256,
    #[serde(rename = "RSA_Decryption_OAEP_SHA384")]
    RsaDecryptionOaepSha384,
    #[serde(rename = "RSA_Decryption_OAEP_SHA512")]
    RsaDecryptionOaepSha512,
    #[serde(rename = "RSA_Signature_PKCS1")]
    RsaSignaturePkcs1,
    #[serde(rename = "RSA_Signature_PSS_MD5")]
    RsaSignaturePssMd5,
    #[serde(rename = "RSA_Signature_PSS_SHA1")]
    RsaSignaturePssSha1,
    #[serde(rename = "RSA_Signature_PSS_SHA224")]
    RsaSignaturePssSha224,
    #[serde(rename = "RSA_Signature_PSS_SHA256")]
    RsaSignaturePssSha256,
    #[serde(rename = "RSA_Signature_PSS_SHA384")]
    RsaSignaturePssSha384,
    #[serde(rename = "RSA_Signature_PSS_SHA512")]
    RsaSignaturePssSha512,
    #[serde(rename = "EdDSA_Signature")]
    EdDsaSignature,
    #[serde(rename = "ECDSA_Signature")]
    EcdsaSignature,
    #[serde(rename = "AES_Encryption_CBC")]
    AesEncryptionCbc,
    #[serde(rename = "AES_Decryption_CBC")]
    AesDecryptionCbc,
}

impl std::fmt::Display for KeyMechanism {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::RsaDecryptionRaw => "RSA_Decryption_RAW",
                Self::RsaDecryptionPkcs1 => "RSA_Decryption_PKCS1",
                Self::RsaDecryptionOaepMd5 => "RSA_Decryption_OAEP_MD5",
                Self::RsaDecryptionOaepSha1 => "RSA_Decryption_OAEP_SHA1",
                Self::RsaDecryptionOaepSha224 => "RSA_Decryption_OAEP_SHA224",
                Self::RsaDecryptionOaepSha256 => "RSA_Decryption_OAEP_SHA256",
                Self::RsaDecryptionOaepSha384 => "RSA_Decryption_OAEP_SHA384",
                Self::RsaDecryptionOaepSha512 => "RSA_Decryption_OAEP_SHA512",
                Self::RsaSignaturePkcs1 => "RSA_Signature_PKCS1",
                Self::RsaSignaturePssMd5 => "RSA_Signature_PSS_MD5",
                Self::RsaSignaturePssSha1 => "RSA_Signature_PSS_SHA1",
                Self::RsaSignaturePssSha224 => "RSA_Signature_PSS_SHA224",
                Self::RsaSignaturePssSha256 => "RSA_Signature_PSS_SHA256",
                Self::RsaSignaturePssSha384 => "RSA_Signature_PSS_SHA384",
                Self::RsaSignaturePssSha512 => "RSA_Signature_PSS_SHA512",
                Self::EdDsaSignature => "EdDSA_Signature",
                Self::EcdsaSignature => "ECDSA_Signature",
                Self::AesEncryptionCbc => "AES_Encryption_CBC",
                Self::AesDecryptionCbc => "AES_Decryption_CBC",
            }
        )
    }
}

impl Default for KeyMechanism {
    fn default() -> KeyMechanism {
        Self::RsaDecryptionRaw
    }
}
