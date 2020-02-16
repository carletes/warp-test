pub mod interfaces {
    use crate::models::Interfaces;
    use std::convert::Infallible;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use warp::http::StatusCode;
    use warp::reply;
    use warp::Reply;

    pub async fn list(ifaces: Arc<Mutex<impl Interfaces>>) -> Result<impl Reply, Infallible> {
        let ifaces = ifaces.lock().await;

        Ok(format!("{:?}", ifaces.all()))
    }

    pub async fn get(
        name: String,
        ifaces: Arc<Mutex<impl Interfaces>>,
    ) -> Result<impl Reply, Infallible> {
        let ifaces = ifaces.lock().await;

        match ifaces.get(&name) {
            Ok(Some(iface)) => Ok(reply::with_status(format!("{:?}", iface), StatusCode::OK)),
            Ok(None) => Ok(reply::with_status("".to_string(), StatusCode::NOT_FOUND)),
            Err(err) => Ok(reply::with_status(
                format!("{:?}", err),
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }
}
