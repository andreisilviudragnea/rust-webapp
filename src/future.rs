use std::pin::pin;

async fn func1() -> usize {
    0
}

#[tokio::test]
async fn test_mut_future() {
    let mut fut = pin!(func1());
    let f = &mut fut;
    println!("{}", f.await);
}
