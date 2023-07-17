#![allow(dead_code, unused_variables)]

use std::{future::Future, pin::Pin, task::Poll};

use axum::{
    body::Body,
    http::{Method, Request},
    response::{IntoResponse, Response},
};
// use prometheus_client::metrics::counter::Counter;
// use prometheus_client::metrics::family::Family;
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

pub(crate) struct ApiLogServiceError<E> {
    pub(crate) inner: E,
}

impl<S> Service<Request<Body>> for ApiLogService<S>
where
    S: Service<Request<Body>> + Send + Clone + 'static,
    <S as Service<Request<Body>>>::Future: Send + 'static,
    <S as Service<Request<Body>>>::Response: IntoResponse,
    <S as Service<Request<Body>>>::Error: IntoResponse,
{
    type Response = Response;

    type Error = ApiLogServiceError<S::Error>;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        match self.inner.poll_ready(cx) {
            std::task::Poll::Ready(res) => match res {
                Ok(_) => Poll::Ready(Ok(())),
                Err(err) => Poll::Ready(Err(ApiLogServiceError { inner: err })),
            },
            std::task::Poll::Pending => Poll::Pending,
        }
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        println!("path: {}", req.uri().path());
        // req.method()
        let mut cloned_self = self.clone();
        let fut = async move {
            let result = cloned_self.inner.call(req).await;
            println!("bar");
            match result {
                Ok(res) => {
                    let (parts, body) = res.into_response().into_parts();
                    println!("succes, status: {}", parts.status);
                    Ok(Response::from_parts(parts, body))
                }
                Err(err_result) => {
                    let (parts, body) = err_result.into_response().into_parts();
                    println!("error_result, status: {}", parts.status);
                    Ok(Response::from_parts(parts, body))
                }
            }
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
