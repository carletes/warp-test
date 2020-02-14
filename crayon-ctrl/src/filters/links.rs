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

    use crate::models::{Interface, Interfaces};
    use std::collections::HashMap;

    struct MockInterfaces {
        ifaces: HashMap<String, Interface>,
    }

    impl Interfaces for MockInterfaces {
        fn all(&self) -> Vec<Interface> {
            let mut ret = Vec::with_capacity(self.ifaces.len());

            for iface in self.ifaces.values() {
                ret.push(iface.clone());
            }
            ret
        }

        fn create(&mut self, name: &str) -> Result<Interface, String> {
            if self.ifaces.contains_key(name) {
                Err(format!("{}: Already exists", name))
            } else {
                let k = String::from(name);
                let iface = Interface {
                    name: k.clone(),
                    addr: "127.0.0.1".to_string(),
                    prefix: 8,
                };
                let ret = iface.clone();
                self.ifaces.insert(k, iface);
                Ok(ret)
            }
        }

        fn delete(&mut self, name: &str) -> Result<bool, String> {
            match self.ifaces.remove(name) {
                Some(_) => Ok(true),
                None => Ok(false),
            }
        }

        fn get(&self, name: &str) -> Result<Option<Interface>, String> {
            match self.ifaces.get(name) {
                Some(iface) => Ok(Some(iface.clone())),
                None => Ok(None),
            }
        }

        fn modify(&self, iface: Interface) -> Result<bool, String> {
            Err(format!("TODO: ip link modify {:?}", iface))
        }
    }
}
