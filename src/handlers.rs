pub mod interfaces {
    use crate::models::Interfaces;
    use std::convert::Infallible;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    pub async fn list<T: Interfaces>(
        ifaces: Arc<Mutex<T>>,
    ) -> Result<impl warp::Reply, Infallible> {
        let ifaces = ifaces.lock().await;

        Ok(format!("{:?}", ifaces.all()))
    }
}
