use std::net::SocketAddr;
use std::time::Duration;

use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;
use tokio::task::block_in_place;
use tracing::metadata::LevelFilter;

pub(crate) async fn axum_main() {
    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::TRACE)
        .finish();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/hello", get(hello))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

lazy_static! {
    pub static ref RT: Runtime = Runtime::new().unwrap();
}

pub fn block<Fu>(f: Fu) -> Fu::Output
where
    Fu: std::future::Future,
{
    match tokio::runtime::Handle::try_current() {
        Ok(handle) => block_in_place(|| handle.block_on(f)),
        Err(_) => RT.block_on(f),
    }
}

#[axum_macros::debug_handler]
// basic handler that responds with a static string
async fn root() -> &'static str {
    block(async {
        tokio::time::sleep(Duration::from_secs(15)).await;
    });
    println!("there");
    block(async {
        tokio::time::sleep(Duration::from_secs(1)).await;
    });
    "Hello, World!"
}

#[axum_macros::debug_handler]
// basic handler that responds with a static string
async fn hello() -> &'static str {
    "Hello, World!"
}

#[axum_macros::debug_handler]
async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // let _rc = std::rc::Rc::new(());

    root().await;

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
