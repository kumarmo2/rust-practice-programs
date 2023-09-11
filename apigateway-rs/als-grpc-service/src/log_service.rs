use std::sync::Arc;

use prometheus_client::{
    encoding::EncodeLabelSet,
    metrics::{counter::Counter, family::Family},
};
use tokio_stream::StreamExt;
use tonic::async_trait;

use crate::envoy::{
    self,
    config::core::v3::RequestMethod,
    service::accesslog::v3::{
        access_log_service_server::AccessLogService, stream_access_logs_message::LogEntries,
    },
};
const HTTP_METHOD_UNSPECIFIED: i32 = RequestMethod::MethodUnspecified as i32;
const HTTP_GET: i32 = RequestMethod::Get as i32;
const HTTP_HEAD: i32 = RequestMethod::Head as i32;
const HTTP_POST: i32 = RequestMethod::Post as i32;
const HTTP_PUT: i32 = RequestMethod::Put as i32;
const HTTP_DELETE: i32 = RequestMethod::Delete as i32;
const HTTP_CONNECT: i32 = RequestMethod::Connect as i32;
const HTTP_OPTIONS: i32 = RequestMethod::Options as i32;
const HTTP_TRACE: i32 = RequestMethod::Trace as i32;
const HTTP_PATCH: i32 = RequestMethod::Patch as i32;

fn get_method(method: i32) -> &'static str {
    match method {
        HTTP_METHOD_UNSPECIFIED => RequestMethod::MethodUnspecified.as_str_name(),
        HTTP_GET => RequestMethod::Get.as_str_name(),
        HTTP_HEAD => RequestMethod::Head.as_str_name(),
        HTTP_POST => RequestMethod::Post.as_str_name(),
        HTTP_PUT => RequestMethod::Put.as_str_name(),
        HTTP_DELETE => RequestMethod::Delete.as_str_name(),
        HTTP_CONNECT => RequestMethod::Connect.as_str_name(),
        HTTP_OPTIONS => RequestMethod::Options.as_str_name(),
        HTTP_TRACE => RequestMethod::Trace.as_str_name(),
        HTTP_PATCH => RequestMethod::Patch.as_str_name(),
        _ => "not_mapped",
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug, EncodeLabelSet)]
pub(crate) struct HttpRequestCountMetricsLabels {
    path: String,
    method: String,
    host: String,
    response_code: u32,
}

pub(crate) struct CustomAccessLogService {
    pub(crate) http_request_count_metrics: Family<HttpRequestCountMetricsLabels, Counter>,
}

#[async_trait]
impl AccessLogService for CustomAccessLogService {
    async fn stream_access_logs(
        &self,
        request: tonic::Request<
            tonic::Streaming<envoy::service::accesslog::v3::StreamAccessLogsMessage>,
        >,
    ) -> std::result::Result<
        tonic::Response<envoy::service::accesslog::v3::StreamAccessLogsResponse>,
        tonic::Status,
    > {
        let stream = request.into_inner();
        tokio::pin!(stream);

        while let Some(message) = stream.next().await {
            let Ok(message) = message else {
                continue;
            };
            for log in message.log_entries.iter() {
                let LogEntries::HttpLogs(http_access_log_entries) = log else {
                    continue;
                };
                for log_entry in http_access_log_entries.log_entry.iter() {
                    let Some(ref request_properties) = log_entry.request else {
                        continue;
                    };

                    let path = &request_properties.path;
                    let method = get_method(request_properties.request_method);
                    let host = request_properties
                        .request_headers
                        .get(":authority")
                        .map(|h| h.as_str());

                    let Some(ref response) = log_entry.response else {
                        continue;
                    };

                    let response_code_details = &response.response_code_details;
                    let response_code = response.response_code.unwrap_or(0);
                    let response_headers_bytes = response.response_headers_bytes;
                    let lables = HttpRequestCountMetricsLabels {
                        host: host.unwrap_or("").to_string(),
                        response_code,
                        path: path.to_string(),
                        method: method.to_string(),
                    };
                    self.http_request_count_metrics.get_or_create(&lables).inc();
                }
            }
        }
        Ok(tonic::Response::new(
            envoy::service::accesslog::v3::StreamAccessLogsResponse {},
        ))
    }
}
