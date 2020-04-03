use crate::handlers::interfaces;
use crate::models::Interfaces;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{Filter, Rejection, Reply};

// GET /links => JSON list of links
pub fn list(
    ifaces: Arc<Mutex<impl Interfaces + Send>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(warp::path::end())
        .and(with_ifaces(ifaces))
        .and_then(interfaces::list)
}

// DELTE /links/<name> => JSON bool, 200 or 404
pub fn delete(
    ifaces: Arc<Mutex<impl Interfaces + Send>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::delete()
        .and(warp::path!(String))
        .and(with_ifaces(ifaces))
        .and_then(interfaces::delete)
}

// GET /links/<name> => JSON object or 404.
pub fn detail(
    ifaces: Arc<Mutex<impl Interfaces + Send>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(warp::path!(String))
        .and(with_ifaces(ifaces))
        .and_then(interfaces::get)
}

// POST /links {JSON body} => Empty response.
pub fn create(
    ifaces: Arc<Mutex<impl Interfaces + Send>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post()
        .and(warp::path::end())
        .and(warp::body::json())
        .and(with_ifaces(ifaces))
        .and_then(interfaces::create)
}

// PATCH /links/<name> {JSON body} => JSON object or 404.
pub fn modify() -> impl Filter<Extract = impl Reply, Error = Rejection> + Copy {
    warp::patch()
        .and(warp::path::param().and(warp::path::end()))
        .and(warp::body::json())
        .map(|_name: String, _body: String| "")
}

fn with_ifaces<T: Interfaces + Send>(
    ifaces: Arc<Mutex<T>>,
) -> impl Filter<Extract = (Arc<Mutex<T>>,), Error = Infallible> + Clone {
    warp::any().map(move || ifaces.clone())
}
