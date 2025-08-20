use crate::common::context::Context;
use poem::{Endpoint, Request};
use std::sync::Arc;
use tokio::sync::OnceCell;

pub struct CacheLocal<T: Send + Sync + 'static>(pub Arc<OnceCell<T>>);

impl<T: Send + Sync + 'static> CacheLocal<T> {
    fn new() -> Self {
        Self(Arc::new(OnceCell::new()))
    }
}

impl<T: Send + Sync + 'static> Clone for CacheLocal<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

pub async fn init_cache_local<T: Clone + Send + Sync + 'static, E: Endpoint>(
    next: E,
    mut req: Request,
) -> poem::Result<E::Output> {
    req.extensions_mut().insert(CacheLocal::<T>::new());

    next.call(req).await
}

pub trait CacheLocalRequestExt {
    fn cache_local<T: Send + Sync + 'static>(&self) -> Option<&CacheLocal<T>>;
}

impl CacheLocalRequestExt for Request {
    fn cache_local<T: Send + Sync + 'static>(&self) -> Option<&CacheLocal<T>> {
        self.extensions().get::<CacheLocal<T>>()
    }
}

impl CacheLocalRequestExt for Context<'_> {
    fn cache_local<T: Send + Sync + 'static>(&self) -> Option<&CacheLocal<T>> {
        self.req.extensions().get::<CacheLocal<T>>()
    }
}
