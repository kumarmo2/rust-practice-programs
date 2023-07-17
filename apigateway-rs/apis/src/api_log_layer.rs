#![allow(dead_code, unused_variables)]

use std::{future::Future, marker::PhantomData, pin::Pin, task::Poll};

use axum::{
    body::Body,
    http::{Method, Request},
    response::{IntoResponse, Response},
};
use prometheus_client::metrics::family::Family;
use prometheus_client::{
    encoding::{EncodeLabel, EncodeLabelSet, EncodeLabelValue},
    metrics::counter::Counter,
};
use tower::{Layer, Service};

use crate::AppState;

#[derive(Hash, PartialEq, Eq, Clone, Debug, EncodeLabelSet)]

pub(crate) struct ApiRequestsMetricLabels {
    method: String,
    path: String, // TODO: check if it is possible to have &str instead of String.
    status: String,
}

#[derive(Clone)]
pub(crate) struct ApiLogService<S> {
    inner: S,
    counter: Family<ApiRequestsMetricLabels, Counter>,
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
        let method = req.method().to_string();
        let path = req.uri().path().to_string();
        let mut cloned_self = self.clone();
        let fut = async move {
            let result = cloned_self.inner.call(req).await;
            match result {
                Ok(res) => {
                    let (parts, body) = res.into_response().into_parts();
                    let _ = cloned_self
                        .counter
                        .get_or_create(&ApiRequestsMetricLabels {
                            method,
                            path,
                            status: parts.status.to_string(),
                        })
                        .inc();
                    Ok(Response::from_parts(parts, body))
                }
                Err(err_result) => {
                    let (parts, body) = err_result.into_response().into_parts();
                    let _ = cloned_self
                        .counter
                        .get_or_create(&ApiRequestsMetricLabels {
                            method,
                            path,
                            status: parts.status.to_string(),
                        })
                        .inc();
                    Ok(Response::from_parts(parts, body))
                }
            }
        };
        Box::pin(fut)
    }
}

#[derive(Clone)]
pub(crate) struct ApiLogLayer {
    counter: Family<ApiRequestsMetricLabels, Counter>,
}

impl ApiLogLayer {
    pub(crate) fn new(
        counter: Family<ApiRequestsMetricLabels, prometheus_client::metrics::counter::Counter>,
    ) -> Self {
        Self { counter }
    }
}

impl<S> Layer<S> for ApiLogLayer
where
    S: Service<Request<Body>>,
{
    type Service = ApiLogService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ApiLogService {
            inner,
            counter: self.counter.clone(),
        }
    }
}
