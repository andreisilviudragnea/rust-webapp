use log::info;
use notify::event::ModifyKind;
use notify::{recommended_watcher, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

pub fn watch_file_content() -> RecommendedWatcher {
    let mut watcher = recommended_watcher(|event: notify::Result<Event>| match &event {
        Ok(event) => match event.kind {
            EventKind::Modify(ModifyKind::Data(_)) => info!("Modified file {event:?}"),
            _ => info!("Received event {event:?}"),
        },
        _ => info!("Received error event {event:?}"),
    })
    .unwrap();

    watcher
        .watch("Cargo.toml".as_ref(), RecursiveMode::Recursive)
        .unwrap();

    watcher
}
