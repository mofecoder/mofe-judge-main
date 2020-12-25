use bollard::{
    container::{Config, CreateContainerOptions, RemoveContainerOptions},
    errors::Error,
    models::{ContainerCreateResponse, HostConfig},
};

const IMAGE: &str = "cafecoder";

// see https://github.com/fussybeaver/bollard

#[derive(Clone)]
pub struct Docker {
    docker: bollard::Docker,
    pub container_name: String,
    pub ip_address: String,
}

impl Docker {
    pub fn new() -> Result<Docker, Error> {
        let docker = bollard::Docker::connect_with_unix_defaults()?;
        Ok(Docker {
            docker,
            container_name: String::new(),
            ip_address: String::new(),
        })
    }

    pub async fn container_create(&mut self, name: &str) -> Result<ContainerCreateResponse, Error> {
        let options = Some(CreateContainerOptions { name });
        let config = Config {
            image: Some(IMAGE),
            host_config: Some(HostConfig {
                memory: Some(2_147_483_648_i64),
                pids_limit: Some(512_i64),
                ..Default::default()
            }),
            ..Default::default()
        };

        self.container_name = name.to_string();

        let inspect = self
            .docker
            .inspect_container(&self.container_name, None)
            .await?;

        let network_settings = inspect
            .network_settings
            .expect("couldn't get network_settings");

        self.ip_address = network_settings
            .ip_address
            .expect("couldn't get IP address");

        self.docker.create_container(options, config).await
    }

    pub async fn container_remove(&self) -> Result<(), Error> {
        let options = RemoveContainerOptions {
            force: true,
            ..Default::default()
        };

        self.docker
            .remove_container(&self.container_name, Some(options))
            .await
    }
}
