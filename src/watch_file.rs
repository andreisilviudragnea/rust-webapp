use log::info;
use notify::event::ModifyKind;
use notify::{recommended_watcher, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::{Arc, RwLock};

pub fn watch_file_content(path: &str) -> (RecommendedWatcher, Arc<RwLock<Arc<String>>>) {
    let file_content = Arc::new(RwLock::new(Arc::new(
        std::fs::read_to_string(path).unwrap(),
    )));

    let file_content2 = file_content.clone();
    let path2 = path.to_string();

    let mut watcher = recommended_watcher(move |event: notify::Result<Event>| match event {
        Ok(event) => match event.kind {
            EventKind::Modify(ModifyKind::Data(_)) => {
                info!("Received modified file data event {event:?}");
                *file_content2.write().unwrap() =
                    Arc::new(std::fs::read_to_string(&path2).unwrap());
            }
            _ => info!("Received event {event:?}"),
        },
        _ => info!("Received error event {event:?}"),
    })
    .unwrap();

    watcher
        .watch(path.as_ref(), RecursiveMode::Recursive)
        .unwrap();

    (watcher, file_content)
}
