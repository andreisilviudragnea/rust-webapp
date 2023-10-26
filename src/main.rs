#![feature(trait_upcasting)]

use std::time::Duration;

use clap::{Arg, Command};
use kube::Error;
use log::{info, LevelFilter};

use simple_logger::SimpleLogger;

use crate::kubeclient::{KubeClient, KubeClientImpl};
use crate::metadata::print_metadata;
use crate::moka::use_moka;
use crate::prost::{create_large_shirt, deserialize_shirt, serialize_shirt};

mod axum;
#[cfg(test)]
mod future;
mod healthcheck;
#[cfg(test)]
mod inherent_trait_method;
mod kubeclient;
mod metadata;
mod mockall;
mod moka;
mod prost;
#[cfg(test)]
mod variance;
mod watch_file;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Trace)
        .with_threads(true)
        .init()
        .unwrap();

    axum::axum_main().await;

    let (_watcher, file_content) = watch_file::watch_file_content("Cargo.toml");

    let mut prev_content = file_content.read().unwrap().clone();
    let mut n = 100_000;

    while n > 0 {
        let content = file_content.read().unwrap().clone();
        if prev_content != content {
            info!("Content changed {prev_content} {content}");
        }
        prev_content = content;
        n -= 1;
    }

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

    let matches = Command::new("metadata fetch example")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .about("Fetch and print the cluster metadata")
        .arg(
            Arg::new("brokers")
                .short('b')
                .long("brokers")
                .help("Broker list in kafka format")
                .num_args(1)
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
                .num_args(1),
        )
        .arg(
            Arg::new("log-conf")
                .long("log-conf")
                .help("Configure the logging format (example: 'rdkafka=trace')")
                .num_args(1),
        )
        .arg(
            Arg::new("timeout")
                .long("timeout")
                .help("Metadata fetch timeout in milliseconds")
                .num_args(1)
                .default_value("60000"),
        )
        .get_matches();

    let brokers = matches.get_one::<String>("brokers").unwrap();
    let timeout = matches.get_one::<u64>("timeout").unwrap();
    let topic = matches.get_one::<String>("topic").map(|it| it.as_str());
    let fetch_offsets = matches.contains_id("offsets");

    print_metadata(
        brokers,
        topic,
        Duration::from_millis(*timeout),
        fetch_offsets,
    );

    warp::serve(healthcheck::healthcheck_filter())
        .run(([0, 0, 0, 0], 3030))
        .await;

    Ok(())
}
