use crate::conn::ConnectionManager;

use actix_session::Session;
use actix_web::web::Data;

use std::sync::Arc;

pub struct RequestContext {
    connections: Arc<ConnectionManager>,
}

impl RequestContext {
    pub fn new(connections: Arc<ConnectionManager>) -> Self {
        RequestContext {
            connections: Arc::new(ConnectionManager::new()),
        }
    }

    pub fn connections(&self) -> &ConnectionManager {
        &self.connections
    }
}

impl std::fmt::Debug for RequestContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RequestContext")
    }
}

pub fn request_context(
    session: &Session,
    c: &Data<Arc<ConnectionManager>>,
    key: &'static str,
) -> RequestContext {
    let cm = c.get_ref().clone();
    RequestContext::new(cm)
}
