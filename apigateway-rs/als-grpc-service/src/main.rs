#![allow(unused_variables, dead_code)]
mod envoy;
mod xds;

use envoy::service::accesslog::v3::access_log_service_server::AccessLogService;
use tokio_stream::StreamExt;
use tonic::async_trait;

use crate::envoy::config::core::v3::RequestMethod;
use crate::envoy::service::accesslog::v3::access_log_service_server::AccessLogServiceServer;
use crate::envoy::service::accesslog::v3::stream_access_logs_message::LogEntries;
struct CustomAccessLogService;

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
            println!("received access log message");
            for log in message.log_entries.iter() {
                let LogEntries::HttpLogs(http_access_log_entries) = log else {
                    continue;
                };
                for log_entry in http_access_log_entries.log_entry.iter() {
                    // log_entry.request.h
                    let Some(ref request_properties) =  log_entry.request else {
                        continue;
                    };

                    let path = &request_properties.path;
                    let method = get_method(request_properties.request_method);

                    let Some(ref response) = log_entry.response else {
                        continue;
                    };
                    let response_code_details = &response.response_code_details;
                    let response_code = response.response_code.unwrap_or(0);
                    let response_body_bytes = response.response_body_bytes;
                    let response_headers_bytes = response.response_headers_bytes;

                    println!(
                        "path: {}, method: {},  response_code: {}, response_body_bytes: {}, response_headers_bytes: {}",
                        path, method, response_code, response_body_bytes, response_headers_bytes
                    );
                }
            }
        }

        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    tonic::transport::Server::builder()
        .add_service(AccessLogServiceServer::new(CustomAccessLogService {}))
        .serve("0.0.0.0:9001".parse()?)
        .await?;

    Ok(())
}
