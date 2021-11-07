use log::LevelFilter;
use simple_logger::SimpleLogger;

mod healthcheck;

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    warp::serve(healthcheck::healthcheck_filter())
        .run(([0, 0, 0, 0], 8080))
        .await;
}
