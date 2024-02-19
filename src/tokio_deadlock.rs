use tokio::runtime::{Builder, Runtime};

#[test]
fn test_spawn_blocking_from_multi_thread_runtime() {
    let runtime = Runtime::new().unwrap();

    let _guard = runtime.enter();

    tokio::task::spawn_blocking(|| {
        println!("Hello from a blocking task!");
    });

    println!("Hello after blocking task!");
}

#[test]
fn test_spawn_blocking_from_current_thread_runtime() {
    let runtime = Builder::new_current_thread().enable_all().build().unwrap();

    let _guard = runtime.enter();

    tokio::task::spawn_blocking(|| {
        println!("Hello from a blocking task!");
    });

    println!("Hello after blocking task!");
}

#[test]
fn test_block_in_place_from_current_thread_runtime() {
    let runtime = Builder::new_current_thread().enable_all().build().unwrap();

    let _guard = runtime.enter();

    tokio::task::block_in_place(|| {
        println!("Hello from block_in_place!");
    });

    println!("Hello after block_in_place!");
}
