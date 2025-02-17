// Based on:
//   https://gitlab.archlinux.org/archlinux/signstar/-/blob/579131fe6b9db9b8fe1b9ffd3ad6d5e98afae816/nethsm/tests/common/container.rs
//   Author: David Runge <dvzrv@archlinux.org>
//   License: Apache-2.0 OR MIT

use nethsm_sdk_rs::apis::configuration::Configuration;
use rustainers::{
    runner::{RunOption, Runner},
    ExposedPort, ImageName, RunnableContainer, RunnableContainerBuilder, ToRunnableContainer,
    WaitStrategy,
};
use ureq::tls::TlsConfig;

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
        client: ureq::Agent::new_with_config(
            ureq::Agent::config_builder()
                .tls_config(TlsConfig::builder().disable_verification(true).build())
                .build(),
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
