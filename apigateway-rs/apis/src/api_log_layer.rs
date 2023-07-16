#![allow(dead_code, unused_variables)]

use std::{future::Future, pin::Pin};

use axum::{
    body::Body,
    http::{Method, Request},
};
use prometheus_client::metrics::counter::Counter;
use prometheus_client::metrics::family::Family;
use tower::{Layer, Service};

use crate::AppState;

struct ApiRequestsMetricLabels<'a> {
    method: Method,
    path: &'a str,
}

#[derive(Clone)]
pub(crate) struct ApiLogService<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for ApiLogService<S>
where
    S: Service<Request<Body>> + Send + Clone + 'static,
    <S as Service<Request<Body>>>::Future: Send + 'static, // S::Future: Send + 'static,
                                                           // <S as Service<Request<Body>>>::Response: 'static,      // S::Response: Send + 'static,
                                                           // <S as Service<Request<Body>>>::Error: 'static,         // S::Response: Send + 'static,
                                                           // S::Error: Send + 'static,
{
    type Response = S::Response;

    type Error = S::Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        println!("path: {}", req.uri().path());
        // req.method()
        let mut cloned_self = self.clone();
        let fut = async move {
            let result = cloned_self.inner.call(req).await;
            println!("bar");
            result
        };
        Box::pin(fut)
    }
}

#[derive(Clone)]
pub(crate) struct ApiLogLayer {
    app_state: AppState,
}

impl ApiLogLayer {
    pub(crate) fn new(app_state: AppState) -> Self {
        Self { app_state }
    }
}

impl<S> Layer<S> for ApiLogLayer
where
    S: Service<Request<Body>>,
{
    type Service = ApiLogService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ApiLogService { inner }
    }
}
