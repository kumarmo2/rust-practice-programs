#![allow(unused_imports, unused_variables, unused_mut, dead_code)]
use prometheus_client::encoding::text::encode;
use prometheus_client::encoding::EncodeLabelSet;
use prometheus_client::metrics::counter::Counter;
use prometheus_client::metrics::family::Family;
use prometheus_client::registry::Registry;

#[derive(Clone, Hash, PartialEq, Eq, Debug, EncodeLabelSet)]
struct Labels {
    method: String,
    path: String,
}

fn main() {
    println!("Hello, world from practice-scratch!");

    let mut registry = Registry::default();

    let requests_counter = Family::<Labels, Counter>::default();

    registry.register(
        "http_requests",
        "counts number of http request",
        requests_counter.clone(),
    );

    for i in 1..=10 {
        requests_counter
            .get_or_create(&Labels {
                method: "get".to_string(),
                path: format!("/api/sdf/{}", i),
            })
            .inc();
        let mut buffer = String::new();

        encode(&mut buffer, &registry).unwrap();

        println!("metrics: {}", buffer);
    }
}
