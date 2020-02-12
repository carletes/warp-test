use warp::Filter;

#[tokio::main]
async fn main() {
    env_logger::init();

    // GET /health => Emty 200 response.
    let health = warp::get().and(warp::path("health").map(|| "").and(warp::path::end()));

    let routes = health;
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
