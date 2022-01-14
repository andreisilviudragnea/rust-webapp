use std::time::Duration;

use clap::{App, Arg};
use kube::Error;
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;

use crate::kubeclient::{KubeClient, KubeClientImpl};
use crate::metadata::print_metadata;
use crate::moka::use_moka;
use crate::prost::{create_large_shirt, deserialize_shirt, serialize_shirt};

mod healthcheck;
mod kubeclient;
mod metadata;
mod mockall;
mod moka;
mod prost;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    let shirt = create_large_shirt("red".to_string());

    info!("Shirt: {:?}", shirt);
    info!(
        "Deserialized shirt: {:?}",
        deserialize_shirt(&serialize_shirt(&shirt))
    );

    use_moka().await;

    let kube_client = KubeClientImpl::new("default").await?;

    let config_map = kube_client.get_config_map("config_map").await?;

    info!("Config map: {:?}", config_map);

    let matches = App::new("metadata fetch example")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .about("Fetch and print the cluster metadata")
        .arg(
            Arg::new("brokers")
                .short('b')
                .long("brokers")
                .help("Broker list in kafka format")
                .takes_value(true)
                .default_value("localhost:9092"),
        )
        .arg(
            Arg::new("offsets")
                .long("offsets")
                .help("Enables offset fetching"),
        )
        .arg(
            Arg::new("topic")
                .long("topic")
                .help("Only fetch the metadata of the specified topic")
                .takes_value(true),
        )
        .arg(
            Arg::new("log-conf")
                .long("log-conf")
                .help("Configure the logging format (example: 'rdkafka=trace')")
                .takes_value(true),
        )
        .arg(
            Arg::new("timeout")
                .long("timeout")
                .help("Metadata fetch timeout in milliseconds")
                .takes_value(true)
                .default_value("60000"),
        )
        .get_matches();

    let brokers = matches.value_of("brokers").unwrap();
    let timeout = matches.value_of_t("timeout").unwrap();
    let topic = matches.value_of("topic");
    let fetch_offsets = matches.is_present("offsets");

    print_metadata(
        brokers,
        topic,
        Duration::from_millis(timeout),
        fetch_offsets,
    );

    warp::serve(healthcheck::healthcheck_filter())
        .run(([0, 0, 0, 0], 3030))
        .await;

    Ok(())
}
