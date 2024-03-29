use std::time::Duration;

use log::trace;

use rdkafka::config::ClientConfig;
use rdkafka::consumer::{BaseConsumer, Consumer};

pub(crate) fn print_metadata(
    brokers: &str,
    topic: Option<&str>,
    timeout: Duration,
    fetch_offsets: bool,
) {
    let consumer: BaseConsumer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .create()
        .expect("Consumer creation failed");

    trace!("Consumer created");

    let metadata = consumer
        .fetch_metadata(topic, timeout)
        .expect("Failed to fetch metadata");

    let mut message_count = 0;

    println!("Cluster information:");
    println!("  Broker count: {}", metadata.brokers().len());
    println!("  Topics count: {}", metadata.topics().len());
    println!("  Metadata broker name: {}", metadata.orig_broker_name());
    println!("  Metadata broker id: {}\n", metadata.orig_broker_id());

    println!("Brokers:");
    for broker in metadata.brokers() {
        println!(
            "  Id: {}  Host: {}:{}  ",
            broker.id(),
            broker.host(),
            broker.port()
        );
    }

    println!("\nTopics:");
    for topic in metadata.topics() {
        println!("  Topic: {}  Err: {:?}", topic.name(), topic.error());
        for partition in topic.partitions() {
            println!(
                "     Partition: {}  Leader: {}  Replicas: {:?}  ISR: {:?}  Err: {:?}",
                partition.id(),
                partition.leader(),
                partition.replicas(),
                partition.isr(),
                partition.error()
            );
            if fetch_offsets {
                let (low, high) = consumer
                    .fetch_watermarks(topic.name(), partition.id(), Duration::from_secs(1))
                    .unwrap_or((-1, -1));
                println!(
                    "       Low watermark: {low}  High watermark: {high} (difference: {})",
                    high - low
                );
                message_count += high - low;
            }
        }
        if fetch_offsets {
            println!("     Total message count: {message_count}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::LevelFilter;
    use rdkafka::producer::ThreadedProducer;
    use simple_logger::SimpleLogger;

    #[test]
    fn test_threaded_producer_statistics_interval_ms_race_condition() -> anyhow::Result<()> {
        SimpleLogger::new()
            .with_level(LevelFilter::Trace)
            .with_threads(true)
            .init()
            .unwrap();

        // statistics.interval.ms needs to be more than 100,
        // because of https://github.com/fede1024/rust-rdkafka/blob/master/src/producer/base_producer.rs#L538
        let _producer: ThreadedProducer<_> = ClientConfig::new()
            .set("statistics.interval.ms", "110")
            .create()?;
        Ok(())
    }
}
