mod utils;

use std::collections::BTreeSet;

use chrono::Utc;
use nethsm_sdk_rs::{
    apis::{configuration::Configuration, default_api, Error},
    models::{
        BackupPassphraseConfig, KeyGenerateRequestData, KeyMechanism, KeyType,
        ProvisionRequestData, RestoreRequestArguments, SystemState, UnlockRequestData,
        UserPostData, UserRole,
    },
};

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

#[tokio::test]
async fn test_restore() {
    let admin_passphrase = "adminadmin";
    let backup_passphrase = "backupbackup";
    let unlock_passphrase = "unlockunlock";

    let (generated_keys, backup) = utils::with_container(|mut config| {
        let request = ProvisionRequestData {
            unlock_passphrase: unlock_passphrase.to_owned(),
            admin_passphrase: admin_passphrase.to_owned(),
            system_time: Utc::now().to_rfc3339(),
        };
        default_api::provision_post(&config, request).unwrap();

        config.basic_auth = Some(("admin".to_owned(), Some(admin_passphrase.to_owned())));

        let request = KeyGenerateRequestData {
            r#type: KeyType::Rsa,
            length: Some(2048),
            mechanisms: vec![KeyMechanism::RsaDecryptionRaw],
            ..Default::default()
        };
        let key_id = default_api::keys_generate_post(&config, request)
            .unwrap()
            .entity
            .id;
        let keys = BTreeSet::from([key_id.clone()]);

        assert_eq!(list_keys(&config), keys);

        let request = BackupPassphraseConfig {
            new_passphrase: backup_passphrase.to_owned(),
            current_passphrase: String::new(),
        };
        default_api::config_backup_passphrase_put(&config, request).unwrap();

        let request = UserPostData {
            real_name: "Backup User".to_owned(),
            role: UserRole::Backup,
            passphrase: backup_passphrase.to_owned(),
        };
        default_api::users_user_id_put(&config, "backup", request).unwrap();

        config.basic_auth = Some(("backup".to_owned(), Some(backup_passphrase.to_owned())));

        let backup = default_api::system_backup_post(&config).unwrap().entity;

        config.basic_auth = Some(("admin".to_owned(), Some(admin_passphrase.to_owned())));

        default_api::keys_key_id_delete(&config, &key_id).unwrap();
        assert_eq!(list_keys(&config), BTreeSet::default());

        let request = RestoreRequestArguments {
            backup_passphrase: Some(backup_passphrase.to_owned()),
            system_time: Some(Utc::now().to_rfc3339()),
        };
        default_api::system_restore_post(&config, Some(request), Some(backup.clone())).unwrap();

        assert_eq!(list_keys(&config), keys);

        (keys, backup)
    })
    .await;

    let restored_keys = utils::with_container(|mut config| {
        let state = default_api::health_state_get(&config).unwrap().entity.state;
        assert_eq!(state, SystemState::Unprovisioned);

        let request = RestoreRequestArguments {
            backup_passphrase: Some(backup_passphrase.to_owned()),
            system_time: Some(Utc::now().to_rfc3339()),
        };
        default_api::system_restore_post(&config, Some(request), Some(backup)).unwrap();

        let state = default_api::health_state_get(&config).unwrap().entity.state;
        assert_eq!(state, SystemState::Locked);

        let request = UnlockRequestData {
            passphrase: unlock_passphrase.to_owned(),
        };
        default_api::unlock_post(&config, request).unwrap();

        config.basic_auth = Some(("admin".to_owned(), Some(admin_passphrase.to_owned())));

        list_keys(&config)
    })
    .await;

    assert_eq!(generated_keys, restored_keys);
}

fn list_keys(config: &Configuration) -> BTreeSet<String> {
    default_api::keys_get(&config, None)
        .unwrap()
        .entity
        .into_iter()
        .map(|item| item.id)
        .collect()
}
