pub mod interfaces {
    use crate::models::{Interface, Interfaces};
    use std::convert::Infallible;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use warp::http::StatusCode;
    use warp::reply;
    use warp::Reply;

    pub async fn list(ifaces: Arc<Mutex<impl Interfaces>>) -> Result<impl Reply, Infallible> {
        let ifaces = ifaces.lock().await;

        match ifaces.all() {
            Ok(ifaces) => Ok(reply::with_status(reply::json(&ifaces), StatusCode::OK)),
            Err(err) => Ok(reply::with_status(
                reply::json(&err),
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }

    pub async fn get(
        name: String,
        ifaces: Arc<Mutex<impl Interfaces>>,
    ) -> Result<impl Reply, Infallible> {
        let ifaces = ifaces.lock().await;

        match ifaces.get(&name) {
            Ok(Some(iface)) => Ok(reply::with_status(reply::json(&iface), StatusCode::OK)),
            Ok(None) => Ok(reply::with_status(
                reply::json(&name),
                StatusCode::NOT_FOUND,
            )),
            Err(err) => Ok(reply::with_status(
                reply::json(&err),
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }

    pub async fn create(
        iface: Interface,
        ifaces: Arc<Mutex<impl Interfaces>>,
    ) -> Result<impl Reply, Infallible> {
        let ifaces = ifaces.lock().await;

        match ifaces.create(iface.name()) {
            Ok(iface) => Ok(reply::with_status(reply::json(&iface), StatusCode::OK)),

            Err(err) => Ok(reply::with_status(
                reply::json(&err),
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }
}
