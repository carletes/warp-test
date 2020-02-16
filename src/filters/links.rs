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
pub fn create() -> impl Filter<Extract = impl Reply, Error = Rejection> + Copy {
    warp::post()
        .and(warp::path::end())
        .and(warp::body::json())
        .map(|_body: String| "")
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::test;

    // #[test]
    // fn list_ifaces() {
    //     let ifaces = test::interfaces();
    //     assert_eq!(ifaces.all().unwrap().is_empty(), true);

    //     let iface = ifaces.create("lo").unwrap();
    //     let all = ifaces.all().unwrap();
    //     assert_eq!(all.len(), 1);
    //     assert_eq!(all[0], iface);

    //     ifaces.delete("lo").unwrap();
    //     assert_eq!(ifaces.all().unwrap().is_empty(), true);
    // }

    // #[test]
    // fn no_dups() {
    //     let ifaces = test::interfaces();
    //     ifaces.create("lo").unwrap();

    //     let err = ifaces.create("lo").unwrap_err();
    //     assert_eq!(err, "lo: Already exists");
    // }

    // #[test]
    // fn delete_missing() {
    //     let ifaces = test::interfaces();
    //     assert_eq!(ifaces.delete("lo").unwrap(), false);

    //     ifaces.create("lo").unwrap();
    //     assert_eq!(ifaces.delete("lo").unwrap(), true);

    //     assert_eq!(ifaces.delete("lo").unwrap(), false);
    // }

    #[tokio::test]
    async fn test_list() {
        let ifaces = test::interfaces();
        let f = list(ifaces);

        let res = warp::test::request()
            .method("GET")
            .path("/")
            .reply(&f)
            .await;

        assert_eq!(res.status(), 200);
        assert_eq!(res.body(), "Ok([])");
    }

    #[tokio::test]
    async fn test_detail() {
        let ifaces = test::interfaces();
        let f = detail(ifaces);

        let res = warp::test::request()
            .method("GET")
            .path("/foo")
            .reply(&f)
            .await;
        assert_eq!(res.status(), 404);
    }
}
