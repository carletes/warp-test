use warp::Filter;

#[tokio::main]
async fn main() {
    env_logger::init();

    // GET /health => Empty 200 response.
    let health = warp::get().and(warp::path("health").map(|| "").and(warp::path::end()));

    let links = warp::path("links").and(
        links::create()
            .or(links::detail())
            .or(links::list())
            .or(links::modify()),
    );

    let routes = health.or(links);
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

mod links {
    use warp::{Filter, Rejection, Reply};

    /// GET /links => JSON list of links
    pub fn list() -> impl Filter<Extract = impl Reply, Error = Rejection> + Copy {
        warp::get().and(warp::path::end()).map(|| "[1, 2, 3]")
    }

    /// GET /links/<name> => JSON object or 404.
    pub fn detail() -> impl Filter<Extract = (String,), Error = Rejection> + Copy {
        warp::get()
            .and(warp::path::param().and(warp::path::end()))
            .map(|name: String| format!("id: {}", name))
    }

    /// POST /links {JSON body} => Empty response.
    pub fn create() -> impl Filter<Extract = impl Reply, Error = Rejection> + Copy {
        warp::post()
            .and(warp::path::end())
            .and(warp::body::json())
            .map(|_body: String| "")
    }

    /// PATCH /links/<name> {JSON body} => JSON object or 404.
    pub fn modify() -> impl Filter<Extract = impl Reply, Error = Rejection> + Copy {
        warp::patch()
            .and(warp::path::param().and(warp::path::end()))
            .and(warp::body::json())
            .map(|_name: String, _body: String| "")
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[tokio::test]
        async fn test_list() {
            let f = list();

            let res = warp::test::request()
                .method("GET")
                .path("/")
                .reply(&f)
                .await;

            assert_eq!(res.status(), 200);
            assert_eq!(res.body(), "[1, 2, 3]");
        }

        #[tokio::test]
        async fn test_detail() {
            let f = detail();

            let res = warp::test::request()
                .method("GET")
                .path("/foo")
                .reply(&f)
                .await;
            assert_eq!(res.status(), 200);
            assert_eq!(res.body(), "id: foo");

            let res = warp::test::request()
                .method("GET")
                .path("/foo/bar")
                .reply(&f)
                .await;
            assert_eq!(res.status(), 404);
        }
    }
}
