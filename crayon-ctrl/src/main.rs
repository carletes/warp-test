use warp::Filter;

mod links;

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
