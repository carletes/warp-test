pub mod interfaces {
    use crate::models::Interfaces;
    use std::convert::Infallible;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use warp::Reply;

    pub async fn list(ifaces: Arc<Mutex<impl Interfaces>>) -> Result<impl Reply, Infallible> {
        let ifaces = ifaces.lock().await;

        Ok(format!("{:?}", ifaces.all()))
    }
}
