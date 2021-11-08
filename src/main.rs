use kube::Error;
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;

use crate::kubeclient::KubeClient;
use crate::kubeclient::KubeClientImpl;

mod healthcheck;
mod kubeclient;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    let kube_client = KubeClientImpl::new("default").await?;

    let config_map = kube_client.get_config_map("config_map").await?;

    info!("Config map: {:?}", config_map);

    warp::serve(healthcheck::healthcheck_filter())
        .run(([0, 0, 0, 0], 8080))
        .await;

    Ok(())
}
