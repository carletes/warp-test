use warp::{Filter, Rejection, Reply};

// GET /links => JSON list of links
pub fn list() -> impl Filter<Extract = impl Reply, Error = Rejection> + Copy {
    warp::get().and(warp::path::end()).map(|| "[1, 2, 3]")
}

// GET /links/<name> => JSON object or 404.
pub fn detail() -> impl Filter<Extract = (String,), Error = Rejection> + Copy {
    warp::get()
        .and(warp::path::param().and(warp::path::end()))
        .map(|name: String| format!("id: {}", name))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::test::MockInterfaces;
    use crate::models::Interfaces;

    #[test]
    fn list_ifaces() {
        let ifaces = MockInterfaces::new();
        assert_eq!(ifaces.all().unwrap().is_empty(), true);

        let iface = ifaces.create("pepe").unwrap();
        let all = ifaces.all().unwrap();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0], iface);

        ifaces.delete("pepe").unwrap();
        assert_eq!(ifaces.all().unwrap().is_empty(), true);
    }

    #[test]
    fn no_dups() {
        let ifaces = MockInterfaces::new();
        ifaces.create("pepe").unwrap();

        let err = ifaces.create("pepe").unwrap_err();
        assert_eq!(err, "pepe: Already exists");
    }

    #[test]
    fn delete_missing() {
        let ifaces = MockInterfaces::new();
        assert_eq!(ifaces.delete("pepe").unwrap(), false);

        ifaces.create("pepe").unwrap();
        assert_eq!(ifaces.delete("pepe").unwrap(), true);

        assert_eq!(ifaces.delete("pepe").unwrap(), false);
    }

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
