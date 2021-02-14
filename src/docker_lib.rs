use crate::models::*;
use bollard::{
    container::{Config, CreateContainerOptions, RemoveContainerOptions},
    errors::Error,
    models::{ContainerCreateResponse, HostConfig},
};
use hyper::{Body, Client, Method, Request};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

const IMAGE: &str = "cafecoder";

// see https://github.com/fussybeaver/bollard

#[derive(Clone)]
pub struct Docker {
    docker: bollard::Docker,
    pub container_name: String,
    pub ip_address: String,
    hashmap: Arc<Mutex<HashMap<String, CmdResultJSON>>>,
}

impl Docker {
    pub fn new(hashmap: Arc<Mutex<HashMap<String, CmdResultJSON>>>) -> Result<Docker, Error> {
        let docker = bollard::Docker::connect_with_unix_defaults()?;
        Ok(Docker {
            docker,
            container_name: String::new(),
            ip_address: String::new(),
            hashmap,
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

    pub async fn request(&self, req: RequestJSON) -> Result<CmdResultJSON, anyhow::Error> {
        let payload = serde_json::to_string(&req)?;

        let request = Request::builder()
            .method(Method::POST)
            .uri(format!("http://{}:8080/request", &self.ip_address))
            .body(Body::from(payload))?;

        let cli = Client::new();
        let resp = cli.request(request).await?;

        loop {
            let mp = self.hashmap.lock().unwrap();
            if mp.get(&req.session_id).is_some() {
                break;
            }
        }

        let cmd_result: CmdResultJSON = self
            .hashmap
            .lock()
            .unwrap()
            .remove(&req.session_id)
            .unwrap()
            .clone();

        Ok(cmd_result)
    }
}
/*
#[cfg(test)]
mod tests {
    use std::thread::sleep;

    use crate::docker_lib::*;
    use crate::server::server;

    #[tokio::test(flavor = "multi_thread")]
    async fn docker_test() -> Result<(), anyhow::Error> {
        let json_map: HashMap<String, CmdResultJSON> = HashMap::new();
        let json_map = Arc::new(Mutex::new(json_map));

        let json_map_ = json_map.clone();
        tokio::spawn(async move {
            if let Err(e) = server(json_map_.clone()).await {
                eprintln!("{:?}", e);
                return;
            }
        });

        let mut docker = Docker::new(json_map.clone())?;

        let resp = docker.container_create("hogehoge").await?;
        dbg!(&resp);

        sleep(std::time::Duration::from_secs(10));

        let _ = docker.container_remove().await?;

        Ok(())
    }
}
*/