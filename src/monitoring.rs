use prometheus::{Encoder, TextEncoder, IntCounterVec, register_int_counter_vec};
use warp::Filter;
use std::convert::Infallible;

lazy_static::lazy_static! {
    pub static ref REQUEST_COUNTER: IntCounterVec = register_int_counter_vec!(
        "player_breeding_system_requests_total",
        "Total number of requests received",
        &["endpoint"]
    ).unwrap();
}

pub async fn metrics_handler() -> Result<impl warp::Reply, Infallible> {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    let response = String::from_utf8(buffer).unwrap();
    Ok(response)
}

pub fn metrics_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("metrics")
        .and(warp::get())
        .and_then(metrics_handler)
}
