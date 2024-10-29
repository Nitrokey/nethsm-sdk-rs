// Based on:
//   https://gitlab.archlinux.org/archlinux/signstar/-/blob/579131fe6b9db9b8fe1b9ffd3ad6d5e98afae816/nethsm/tests/common/container.rs
//   Author: David Runge <dvzrv@archlinux.org>
//   License: Apache-2.0 OR MIT

use std::{
    fmt::Debug,
    io::{Read, Write},
    sync::Arc,
};

use nethsm_sdk_rs::apis::configuration::Configuration;
use rustainers::{
    runner::{RunOption, Runner},
    ExposedPort, ImageName, RunnableContainer, RunnableContainerBuilder, ToRunnableContainer,
    WaitStrategy,
};
use rustls::{
    client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier},
    crypto::{
        self, verify_tls12_signature, verify_tls13_signature, CryptoProvider,
        WebPkiSupportedAlgorithms,
    },
    pki_types::{CertificateDer, ServerName, UnixTime},
    ClientConnection, DigitallySignedStruct, SignatureScheme, StreamOwned,
};
use ureq::{
    resolver::DefaultResolver,
    transport::{
        Buffers, ChainedConnector, ConnectionDetails, Connector, LazyBuffers, NextTimeout,
        TcpConnector, Transport, TransportAdapter,
    },
};

#[derive(Debug)]
struct TlsConnectorAcceptAll;

impl Connector for TlsConnectorAcceptAll {
    fn connect(
        &self,
        details: &ConnectionDetails,
        chained: Option<Box<dyn Transport>>,
    ) -> Result<Option<Box<dyn Transport>>, ureq::Error> {
        let Some(transport) = chained else {
            panic!("Chained connector is required");
        };

        if !details.needs_tls() || transport.is_tls() {
            return Ok(Some(transport));
        }

        let name_borrowed: ServerName<'_> = details
            .uri
            .authority()
            .expect("uri authority for tls")
            .host()
            .try_into()
            .map_err(|_e| ureq::Error::Tls("Rustls invalid dns name error"))?;

        let name = name_borrowed.to_owned();
        let config = rustls_accept_all_config();
        let conn = ClientConnection::new(config, name)?;

        let stream = StreamOwned {
            conn,
            sock: TransportAdapter::new(transport),
        };

        // FIXME: use details.config.(input/output)_buffer_size,
        let buffers = LazyBuffers::new(512 * 1024, 512 * 1024);

        let transport = Box::new(CertIgnoredTransport { buffers, stream });
        Ok(Some(transport))
    }
}

struct CertIgnoredTransport {
    stream: StreamOwned<ClientConnection, TransportAdapter>,
    buffers: LazyBuffers,
}

impl Debug for CertIgnoredTransport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CertIgnoredTransport")
            .finish_non_exhaustive()
    }
}

impl Transport for CertIgnoredTransport {
    fn buffers(&mut self) -> &mut dyn Buffers {
        &mut self.buffers
    }

    fn transmit_output(&mut self, amount: usize, timeout: NextTimeout) -> Result<(), ureq::Error> {
        self.stream.get_mut().set_timeout(timeout);

        let output = &self.buffers.output()[..amount];
        self.stream.write_all(output)?;

        Ok(())
    }

    fn await_input(&mut self, timeout: NextTimeout) -> Result<bool, ureq::Error> {
        if self.buffers.can_use_input() {
            return Ok(true);
        }

        self.stream.get_mut().set_timeout(timeout);

        let input = self.buffers.input_append_buf();
        let amount = self.stream.read(input)?;
        self.buffers.input_appended(amount);

        Ok(amount > 0)
    }

    fn is_open(&mut self) -> bool {
        self.stream.get_mut().get_mut().is_open()
    }

    fn is_tls(&self) -> bool {
        true
    }
}

fn rustls_accept_all_config() -> Arc<rustls::ClientConfig> {
    let provider = Arc::new(rustls::crypto::ring::default_provider());

    let config = rustls::ClientConfig::builder_with_provider(provider.clone())
        .with_protocol_versions(rustls::ALL_VERSIONS)
        .unwrap()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(DisabledVerifier(provider.clone())))
        .with_no_client_auth();

    Arc::new(config)
}

#[derive(Debug)]
struct DisabledVerifier(Arc<CryptoProvider>);

impl ServerCertVerifier for DisabledVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        Ok(rustls::client::danger::ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        verify_tls12_signature(
            message,
            cert,
            dss,
            &self.0.signature_verification_algorithms,
        )
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        verify_tls13_signature(
            message,
            cert,
            dss,
            &self.0.signature_verification_algorithms,
        )
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.0.signature_verification_algorithms.supported_schemes()
    }
}

pub async fn with_container<F: FnOnce(Configuration) -> T, T>(f: F) -> T {
    let _ = env_logger::builder().is_test(true).try_init();

    let runner = Runner::auto().unwrap();
    let options = RunOption::builder().with_remove(true).build();
    let container = runner
        .start_with_options(Image::new(), options)
        .await
        .unwrap();

    let config = Configuration {
        base_path: container.api().await,
        client: ureq::Agent::with_parts(
            ureq::Agent::config_builder().build(),
            ChainedConnector::new([
                Box::new(TcpConnector::default()) as Box<dyn Connector>,
                Box::new(TlsConnectorAcceptAll),
            ]),
            DefaultResolver::default(),
        ),
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
