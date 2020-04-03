use std::sync::Arc;
use tokio::sync::Mutex;

use crate::filters::links;

use crate::models::{Interface, Interfaces};
use std::cell::RefCell;
use std::collections::HashMap;

struct MockInterfaces {
    ifaces: RefCell<HashMap<String, Interface>>,
}

impl Interfaces for MockInterfaces {
    fn all(&self) -> Result<Vec<Interface>, String> {
        Ok(self.ifaces.borrow().values().cloned().collect())
    }

    fn create(&self, name: &str) -> Result<Interface, String> {
        if self.ifaces.borrow().contains_key(name) {
            Err(format!("{}: Already exists", name))
        } else {
            let iface = Interface::new(
                name,
                "127.0.0.1",
                 "255.0.0.0",
            );
            let ret = iface.clone();
            let k = String::from(name);
            self.ifaces.borrow_mut().insert(k, iface);
            Ok(ret)
        }
    }

    fn delete(&self, name: &str) -> Result<bool, String> {
        match self.ifaces.borrow_mut().remove(name) {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    fn get(&self, name: &str) -> Result<Option<Interface>, String> {
        match self.ifaces.borrow().get(name) {
            Some(iface) => Ok(Some(iface.clone())),
            None => Ok(None),
        }
    }

    fn modify(&self, iface: Interface) -> Result<bool, String> {
        Err(format!("TODO: ip link modify {:?}", iface))
    }
}

pub fn interfaces() -> Arc<Mutex<impl Interfaces>> {
    Arc::new(Mutex::new(MockInterfaces {
        ifaces: RefCell::new(HashMap::with_capacity(10)),
    }))
}
//
// #[test]
// fn no_dups() {
//     let ifaces = test::interfaces();
//     ifaces.create("lo").unwrap();

//     let err = ifaces.create("lo").unwrap_err();
//     assert_eq!(err, "lo: Already exists");
// }

#[tokio::test]
async fn no_interfaces() {
    let ifaces = interfaces();
    let f = links::list(ifaces.clone());

    let res = warp::test::request()
        .method("GET")
        .path("/")
        .reply(&f)
        .await;
    assert_eq!(res.status(), 200);
    assert_eq!(res.body(), "[]");
}

#[tokio::test]
async fn some_interfaces() {
    let ifaces = interfaces();
    let f = links::list(ifaces.clone());

    ifaces.lock().await.create("lo").unwrap();
    let res = warp::test::request()
        .method("GET")
        .path("/")
        .reply(&f)
        .await;
    assert_eq!(res.status(), 200);
    assert_eq!(
        res.body(),
        "[{\"name\":\"lo\",\"addr\":\"127.0.0.1\",\"netmask\":\"255.0.0.0\"}]"
    );
}

#[tokio::test]
async fn unknown_interface_detail() {
    let ifaces = interfaces();
    let f = links::detail(ifaces.clone());

    let res = warp::test::request()
        .method("GET")
        .path("/lo")
        .reply(&f)
        .await;
    assert_eq!(res.status(), 404);
}

#[tokio::test]
async fn interface_detail() {
    let ifaces = interfaces();
    let f = links::detail(ifaces.clone());

    ifaces.lock().await.create("lo").unwrap();
    let res = warp::test::request()
        .method("GET")
        .path("/lo")
        .reply(&f)
        .await;
    assert_eq!(res.status(), 200);
    assert_eq!(
        res.body(),
        "{\"name\":\"lo\",\"addr\":\"127.0.0.1\",\"netmask\":\"255.0.0.0\"}"
    );
}

#[tokio::test]
async fn delete_interface() {
    let ifaces = interfaces();
    let f = links::detail(ifaces.clone());

    ifaces.lock().await.create("lo").unwrap();
    let res = warp::test::request()
        .method("GET")
        .path("/lo")
        .reply(&f)
        .await;
    assert_eq!(res.status(), 200);

    let f = links::delete(ifaces.clone());
    let res = warp::test::request()
        .method("DELETE")
        .path("/lo")
        .reply(&f)
        .await;
    assert_eq!(res.status(), 200);

    let f = links::detail(ifaces.clone());
    let res = warp::test::request()
        .method("GET")
        .path("/lo")
        .reply(&f)
        .await;
    assert_eq!(res.status(), 404);
}

#[tokio::test]
async fn delete_unknown_interface() {
    let ifaces = interfaces();
    let f = links::delete(ifaces.clone());
    let res = warp::test::request()
        .method("DELETE")
        .path("/no-such-iface")
        .reply(&f)
        .await;
    assert_eq!(res.status(), 404);
}
