use warp::Filter;

#[tokio::main]
async fn main() {
    env_logger::init();

    // GET /health => Empty 200 response.
    let health = warp::get().and(warp::path("health").map(|| "").and(warp::path::end()));

    let links = warp::path("links").and(
        // GET /links => JSON list of links
        warp::get()
            .and(warp::path::end())
            .map(|| "[1, 2, 3]")
            .or(
                // GET /links/<name> => JSON object or 404.
                warp::get().and(
                    warp::path::param()
                        .and(warp::path::end())
                        .map(|name: String| format!("id: {}", name)),
                ),
            )
            .or(
                // POST /links {JSON body} => Empty response.
                warp::post().and(warp::body::json()).map(|_body: String| ""),
            )
            .or(
                // PATCH /links/<name> {JSON body} => JSON object or 404.
                warp::patch()
                    .and(warp::body::json())
                    .map(|_body: String| ""),
            ),
    );

    let routes = health.or(links);
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
