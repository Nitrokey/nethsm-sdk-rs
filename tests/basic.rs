mod utils;

use nethsm_sdk_rs::apis::{default_api, Error};

#[tokio::test]
async fn test_health_state() {
    utils::with_container(|config| {
        let result = default_api::health_state_get(&config);
        assert!(result.is_ok(), "{result:?}");
    })
    .await
}

#[tokio::test]
async fn test_error() {
    utils::with_container(|config| {
        let err = default_api::keys_get(&config, None).err().unwrap();
        match err {
            Error::ResponseError(content) => {
                assert_eq!(content.status, 412);
                let err = String::from_utf8_lossy(&content.content);
                assert!(
                    err.contains("Service not available"),
                    "unexpected error message: {err}"
                );
            }
            _ => {
                panic!("Unexpected error variant: {err:?}");
            }
        }
    })
    .await
}
