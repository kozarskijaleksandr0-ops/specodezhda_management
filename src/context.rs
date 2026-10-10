use std::sync::Arc;
use async_sqlite::Pool;

#[derive(Clone)]
pub struct RequestContext {
    pub(crate) connection_pool: Arc<Pool>,
}

impl RequestContext {
    pub fn new(connection_pool: Arc<Pool>) -> Self {
        Self { connection_pool }
    }
}