// Based on:
//   https://gitlab.archlinux.org/archlinux/signstar/-/blob/579131fe6b9db9b8fe1b9ffd3ad6d5e98afae816/nethsm/tests/common/container.rs
//   Author: David Runge <dvzrv@archlinux.org>
//   License: Apache-2.0 OR MIT

use std::sync::Arc;

use nethsm_sdk_rs::apis::configuration::Configuration;
use rustainers::{
    runner::{RunOption, Runner},
    ExposedPort, ImageName, RunnableContainer, RunnableContainerBuilder, ToRunnableContainer,
    WaitStrategy,
};
use rustls::{
    client::{
        danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier},
        ClientConfig,
    },
    crypto::{self, WebPkiSupportedAlgorithms},
    pki_types::{CertificateDer, ServerName, UnixTime},
    DigitallySignedStruct, SignatureScheme,
};

pub async fn with_container<F: FnOnce(Configuration) -> T, T>(f: F) -> T {
    let _ = env_logger::builder().is_test(true).try_init();

    let runner = Runner::auto().unwrap();
    let options = RunOption::builder().with_remove(true).build();
    let container = runner
        .start_with_options(Image::new(), options)
        .await
        .unwrap();

    let tls_config = Arc::new(
        ClientConfig::builder()
            .dangerous()
            .with_custom_certificate_verifier(Arc::new(Verifier::default()))
            .with_no_client_auth(),
    );
    let config = Configuration {
        base_path: container.api().await,
        client: ureq::builder().tls_config(tls_config).build(),
        ..Default::default()
    };

    let result = f(config);
    drop(container);
    result
}

struct Image {
    name: ImageName,
    port: ExposedPort,
}

impl Image {
    fn new() -> Self {
        Self {
            name: ImageName::new_with_tag("nitrokey/nethsm", "testing"),
            port: ExposedPort::new(8443),
        }
    }

    async fn api(&self) -> String {
        let port = self.port.host_port().await.unwrap();
        format!("https://localhost:{port}/api/v1")
    }
}

impl ToRunnableContainer for Image {
    fn to_runnable(&self, builder: RunnableContainerBuilder) -> RunnableContainer {
        builder
            .with_image(self.name.clone())
            .with_wait_strategy(WaitStrategy::stderr_contains(
                "listening on 8443/TCP for HTTPS",
            ))
            .with_port_mappings([self.port.clone()])
            .build()
    }
}

#[derive(Debug)]
struct Verifier(WebPkiSupportedAlgorithms);

impl Default for Verifier {
    fn default() -> Self {
        Self(crypto::ring::default_provider().signature_verification_algorithms)
    }
}

impl ServerCertVerifier for Verifier {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        crypto::verify_tls12_signature(message, cert, dss, &self.0)
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        crypto::verify_tls13_signature(message, cert, dss, &self.0)
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.0.supported_schemes()
    }
}
