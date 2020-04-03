use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

mod filters;
mod handlers;
mod models;

#[tokio::main]
async fn main() {
    env_logger::init();

    // GET /health => Empty 200 response.
    let health = warp::get().and(warp::path("health").map(|| "").and(warp::path::end()));

    let ifaces = Arc::new(Mutex::new(models::SystemInterfaces::new()));

    let links = warp::path("links").and(
        filters::links::create(ifaces.clone())
            .or(filters::links::delete(ifaces.clone()))
            .or(filters::links::detail(ifaces.clone()))
            .or(filters::links::list(ifaces.clone()))
            .or(filters::links::modify()),
    );

    let routes = health.or(links);
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

#[cfg(test)]
mod tests;
