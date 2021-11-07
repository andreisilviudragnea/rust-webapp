use warp::{Filter, Rejection, Reply};

pub fn healthcheck_filter() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
    warp::get().and(warp::path!("healthcheck")).map(warp::reply)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_healthcheck_filter() {
        let filter = healthcheck_filter();

        let res = warp::test::request()
            .path("/healthcheck")
            .reply(&filter)
            .await;
        assert_eq!(res.status(), 200);
        assert_eq!(res.body(), "");

        let res = warp::test::request()
            .method("POST")
            .path("/healthcheck")
            .reply(&filter)
            .await;
        assert_eq!(res.status(), 405);
        assert_eq!(res.body(), "HTTP method not allowed");
    }
}
