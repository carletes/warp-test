use warp::Filter;

mod filters;
mod models;

#[tokio::main]
async fn main() {
    env_logger::init();

    // GET /health => Empty 200 response.
    let health = warp::get().and(warp::path("health").map(|| "").and(warp::path::end()));

    let ifaces = models::SystemInterfaces::new();

    let links = warp::path("links").and(
        filters::links::create()
            .or(filters::links::detail())
            .or(filters::links::list())
            .or(filters::links::modify()),
    );

    let routes = health.or(links);
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
