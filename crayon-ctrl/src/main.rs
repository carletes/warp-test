use warp::Filter;

#[tokio::main]
async fn main() {
    env_logger::init();

    // GET /health => Empty 200 response.
    let health = warp::get().and(warp::path("health").map(|| "").and(warp::path::end()));

    // GET /links => JSON list of links
    let links_list = warp::get().and(warp::path::end()).map(|| "[1, 2, 3]");

    // GET /links/<name> => JSON object or 404.
    let links_detalil = warp::get()
        .and(warp::path::param().and(warp::path::end()))
        .map(|name: String| format!("id: {}", name));

    // POST /links {JSON body} => Empty response.
    let links_create = warp::post()
        .and(warp::path::end())
        .and(warp::body::json())
        .map(|_body: String| "");

    // PATCH /links/<name> {JSON body} => JSON object or 404.
    let links_modify = warp::patch()
        .and(warp::path::param().and(warp::path::end()))
        .and(warp::body::json())
        .map(|_name: String, _body: String| "");

    let links = warp::path("links").and(
        links_create
            .or(links_detalil)
            .or(links_list)
            .or(links_modify),
    );

    let routes = health.or(links);
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
