//! Default Compute@Edge template program.
use config::{Config, FileFormat};
use fastly::http::{HeaderValue, Method};
use fastly::{Body, Error, Request, RequestExt, Response, ResponseExt};
/// The name of a backend server associated with this service.
///
/// This should be changed to match the name of your own backend. See the the `Hosts` section of
/// the Fastly WASM service UI for more information.
const BACKEND_NAME: &str = "SpaceXData";
const URI: &str = "https://api.spacexdata.com/v3/launches?filter=rocket/second_stage/payloads/(payload_id,norad_id)&mission_id=6C42550"; 
// const BACKEND_NAME: &str = "n2yo";
// const URI: &str = "https://api.n2yo.com/rest/v1/satellite/tle/39500?apiKey=YBLNQJ-JUG3KB-XS5BRT-1JX2"; 
const LOGGING_ENDPOINT: &str = "get_test_syslog";
/// The entry point for your application.
///
/// This function is triggered when your service receives a client request. It could be used to
/// route based on the request properties (such as method or path), send the request to a backend,
/// make completely new requests, and/or generate synthetic responses.
///
/// If `main` returns an error, a 500 error response will be delivered to the client.
#[fastly::main]
fn main(req: Request<Body>) -> Result<impl ResponseExt, Error> {
    logging_init();
    let req2 = Request::builder().uri(URI).method(Method::GET).body(Body::from(""));
    let req2 = req2.unwrap();
    log::debug!("Request: {} {}", req2.method(), req2.uri());
    let beresp = req2.send(BACKEND_NAME)?;
    //let beresp = req.send(BACKEND_NAME)?;
    let headers = beresp.headers().to_owned();
    let headers2 = beresp.headers().to_owned();
    for (header, header_val) in headers2 {
        log::debug!("Header {:?}: {:?}", header, header_val);
    }
    let (parts, body) = beresp.into_parts();
    let body_str = body.into_string();
    log::debug!("Body: {:?}", body_str);
    let newres = Response::from_parts(parts, Body::from(body_str));
    let default_header_value = &HeaderValue::from_str("0").unwrap();
    let content_type = headers.get("Content-Type").unwrap_or(default_header_value);
    log::debug!("Content Type: {:?}", content_type);
    Ok(newres)
}
/// This function reads the fastly.toml file and gets the deployed version. This is only run at
/// compile time. Since we bump the version number after building (during the deploy) we return
/// the version incremented by one so the version returned will match the deployed version.
fn get_version() -> i32 {
    Config::new()
        .merge(config::File::from_str(
            include_str!("../fastly.toml"), // assumes the existence of fastly.toml
            FileFormat::Toml,
        ))
        .unwrap()
        .get_str("version")
        .unwrap()
        .parse::<i32>()
        .unwrap_or(0)
        + 1
}
// Boiler plate function that I will include in every app until we have something in place that
// doe this.
fn logging_init() {
    log_fastly::Logger::builder()
        .max_level(log::LevelFilter::Debug)
        .default_endpoint(LOGGING_ENDPOINT)
        .init();
    fastly::log::set_panic_endpoint(LOGGING_ENDPOINT).unwrap();
    log::debug!("*******************************************************");
    log::debug!("Get Test Version:{}", get_version());
}
