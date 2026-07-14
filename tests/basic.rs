mod utils;

use std::collections::BTreeSet;

use chrono::Utc;
use nethsm_sdk_rs::{
    apis::{Error, configuration::Configuration, default_api},
    models::{
        BackupPassphraseConfig, KeyGenerateRequestData, KeyMechanism, KeySetLabel, KeyType,
        LogLevel, LoggingConfig, ProvisionRequestData, RestoreRequestArguments, SystemState,
        UnlockRequestData, UserPostData, UserRole,
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
async fn test_error_with_body() {
    utils::with_container(|config| {
        let err = default_api::keys_get(&config, None, None).err().unwrap();
        match err {
            Error::ResponseError(content) => {
                assert_eq!(content.status, 412);
                let err = String::from_utf8_lossy(&content.content);
                assert!(
                    err.contains("Service not available"),
                    "unexpected error message: {err}"
                );
                match content.entity {
                    default_api::KeysGetError::Status412() => {}
                    err => panic!("Unexpected error variant: {err:?}"),
                }
            }
            _ => {
                panic!("Unexpected error variant: {err:?}");
            }
        }
    })
    .await
}

#[tokio::test]
async fn test_labels() {
    let admin_passphrase = "adminadmin";
    let unlock_passphrase = "unlockunlock";

    utils::with_container(|mut config| {
        let request = ProvisionRequestData::new(
            unlock_passphrase.to_owned(),
            admin_passphrase.to_owned(),
            Utc::now().to_rfc3339(),
        );
        default_api::provision_post(&config, request).unwrap();

        config.basic_auth = Some(("admin".to_owned(), Some(admin_passphrase.to_owned())));

        let logging_config = LoggingConfig::new("0.0.0.0".into(), 0, LogLevel::Debug);
        default_api::config_logging_put(&config, logging_config).unwrap();

        let version = utils::version(&config);
        let has_labels = version.major >= 5;

        let mut request =
            KeyGenerateRequestData::new(vec![KeyMechanism::RsaDecryptionRaw], KeyType::Rsa);
        request.length = Some(2048);
        let key_id = default_api::keys_generate_post(&config, request)
            .unwrap()
            .entity
            .id;
        let keys = BTreeSet::from([key_id.clone()]);

        assert_eq!(list_keys(&config, None), keys);
        assert_eq!(list_keys(&config, Some("")), keys);

        if has_labels {
            assert_eq!(list_keys(&config, Some("important")), BTreeSet::new());
            assert_eq!(list_keys(&config, Some("irrelevant")), BTreeSet::new());

            default_api::keys_key_id_label_put(
                &config,
                &key_id,
                KeySetLabel::new("important".to_owned()),
            )
            .unwrap();

            assert_eq!(list_keys(&config, None), keys);
            assert_eq!(list_keys(&config, Some("")), BTreeSet::new());
            assert_eq!(list_keys(&config, Some("important")), keys);
            assert_eq!(list_keys(&config, Some("irrelevant")), BTreeSet::new());
            assert_eq!(list_keys(&config, Some("i")), BTreeSet::new());

            let key = default_api::keys_key_id_get(&config, &key_id)
                .unwrap()
                .entity;
            assert_eq!(key.label.as_deref(), Some("important"));

            default_api::keys_key_id_label_put(
                &config,
                &key_id,
                KeySetLabel::new("important".to_owned()),
            )
            .unwrap();

            default_api::keys_key_id_label_put(
                &config,
                &key_id,
                KeySetLabel::new("irrelevant".to_owned()),
            )
            .unwrap();

            assert_eq!(list_keys(&config, None), keys);
            assert_eq!(list_keys(&config, Some("")), BTreeSet::new());
            assert_eq!(list_keys(&config, Some("important")), BTreeSet::new());
            assert_eq!(list_keys(&config, Some("irrelevant")), keys);
            assert_eq!(list_keys(&config, Some("i")), BTreeSet::new());

            let key = default_api::keys_key_id_get(&config, &key_id)
                .unwrap()
                .entity;
            assert_eq!(key.label.as_deref(), Some("irrelevant"));
        } else {
            assert_eq!(list_keys(&config, Some("important")), keys);
            assert_eq!(list_keys(&config, Some("irrelevant")), keys);

            let key = default_api::keys_key_id_get(&config, &key_id)
                .unwrap()
                .entity;
            assert_eq!(key.label, None);
        }
    })
    .await
}

#[tokio::test]
async fn test_namespaces() {
    let admin_passphrase = "adminadmin";
    let n_admin_passphrase = "admin2admin2";
    let unlock_passphrase = "unlockunlock";

    utils::with_container(|mut config| {
        let request = ProvisionRequestData::new(
            unlock_passphrase.to_owned(),
            admin_passphrase.to_owned(),
            Utc::now().to_rfc3339(),
        );
        default_api::provision_post(&config, request).unwrap();

        config.basic_auth = Some(("admin".to_owned(), Some(admin_passphrase.to_owned())));

        let version = utils::version(&config);
        // namespace support was added in v2.0
        if version.major < 2 {
            return;
        }

        let request = UserPostData::new(
            "N-Admin".to_owned(),
            UserRole::Administrator,
            n_admin_passphrase.to_owned(),
        );
        let user_id = default_api::users_user_id_post(&config, "mynamespace~", request)
            .unwrap()
            .entity
            .id;
        assert!(user_id.starts_with("mynamespace~"));

        assert_eq!(list_namespaces(&config), BTreeSet::new());

        default_api::namespaces_namespace_id_put(&config, "mynamespace").unwrap();

        assert_eq!(
            list_namespaces(&config),
            ["mynamespace".to_owned()].into_iter().collect()
        );

        config.basic_auth = Some((user_id, Some(n_admin_passphrase.to_owned())));

        let mut request =
            KeyGenerateRequestData::new(vec![KeyMechanism::RsaDecryptionRaw], KeyType::Rsa);
        request.length = Some(2048);
        let key_id = default_api::keys_generate_post(&config, request)
            .unwrap()
            .entity
            .id;
        let keys = BTreeSet::from([key_id.clone()]);

        assert_eq!(list_keys(&config, None), keys);

        config.basic_auth = Some(("admin".to_owned(), Some(admin_passphrase.to_owned())));

        assert_eq!(list_keys(&config, None), BTreeSet::new());

        default_api::namespaces_namespace_id_delete(&config, "mynamespace").unwrap();

        assert_eq!(list_namespaces(&config), BTreeSet::new());
    })
    .await
}

#[tokio::test]
async fn test_restore() {
    let admin_passphrase = "adminadmin";
    let backup_passphrase = "backupbackup";
    let unlock_passphrase = "unlockunlock";

    let (generated_keys, backup) = utils::with_container(|mut config| {
        let request = ProvisionRequestData::new(
            unlock_passphrase.to_owned(),
            admin_passphrase.to_owned(),
            Utc::now().to_rfc3339(),
        );
        default_api::provision_post(&config, request).unwrap();

        config.basic_auth = Some(("admin".to_owned(), Some(admin_passphrase.to_owned())));

        let mut request =
            KeyGenerateRequestData::new(vec![KeyMechanism::RsaDecryptionRaw], KeyType::Rsa);
        request.length = Some(2048);
        let key_id = default_api::keys_generate_post(&config, request)
            .unwrap()
            .entity
            .id;
        let keys = BTreeSet::from([key_id.clone()]);

        assert_eq!(list_keys(&config, None), keys);

        let request = BackupPassphraseConfig::new(backup_passphrase.to_owned(), String::new());
        default_api::config_backup_passphrase_put(&config, request).unwrap();

        let request = UserPostData::new(
            "Backup User".to_owned(),
            UserRole::Backup,
            backup_passphrase.to_owned(),
        );
        default_api::users_user_id_put(&config, "backup", request).unwrap();

        config.basic_auth = Some(("backup".to_owned(), Some(backup_passphrase.to_owned())));

        let backup = default_api::system_backup_post(&config).unwrap().entity;

        config.basic_auth = Some(("admin".to_owned(), Some(admin_passphrase.to_owned())));

        default_api::keys_key_id_delete(&config, &key_id).unwrap();
        assert_eq!(list_keys(&config, None), BTreeSet::default());

        let mut request = RestoreRequestArguments::new();
        request.backup_passphrase = Some(backup_passphrase.to_owned());
        request.system_time = Some(Utc::now().to_rfc3339());
        default_api::system_restore_post(&config, Some(request), Some(backup.clone())).unwrap();

        assert_eq!(list_keys(&config, None), keys);

        (keys, backup)
    })
    .await;

    let restored_keys = utils::with_container(|mut config| {
        let state = default_api::health_state_get(&config).unwrap().entity.state;
        assert_eq!(state, SystemState::Unprovisioned);

        let mut request = RestoreRequestArguments::new();
        request.backup_passphrase = Some(backup_passphrase.to_owned());
        request.system_time = Some(Utc::now().to_rfc3339());
        default_api::system_restore_post(&config, Some(request), Some(backup)).unwrap();

        let state = default_api::health_state_get(&config).unwrap().entity.state;
        assert_eq!(state, SystemState::Locked);

        let request = UnlockRequestData::new(unlock_passphrase.to_owned());
        default_api::unlock_post(&config, request).unwrap();

        config.basic_auth = Some(("admin".to_owned(), Some(admin_passphrase.to_owned())));

        list_keys(&config, None)
    })
    .await;

    assert_eq!(generated_keys, restored_keys);
}

fn list_keys(config: &Configuration, label: Option<&str>) -> BTreeSet<String> {
    default_api::keys_get(config, None, label)
        .unwrap()
        .entity
        .into_iter()
        .map(|item| item.id)
        .collect()
}

fn list_namespaces(config: &Configuration) -> BTreeSet<String> {
    default_api::namespaces_get(config)
        .unwrap()
        .entity
        .into_iter()
        .map(|item| item.id)
        .collect()
}
