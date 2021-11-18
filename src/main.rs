use hyper::header::*;
use hyper::Body;
use hyper::Client;
use hyper::Request;
use hyper::Version;
use hyper_timeout::TimeoutConnector;
use hyper_tls::HttpsConnector;
use std::time::Duration;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let https = HttpsConnector::new();

    let mut connector = TimeoutConnector::new(https);
    connector.set_connect_timeout(Some(Duration::from_secs(10)));
    connector.set_read_timeout(Some(Duration::from_secs(10)));
    connector.set_write_timeout(Some(Duration::from_secs(10)));

    let client = Client::builder().build::<_, hyper::Body>(connector);

    for scheme in &["https", "wss"] {
        println!("scheme is {}:", scheme);
        let mut req = Request::builder()
            .version(Version::HTTP_11)
            .method("GET")
            .uri(format!(
                "{}://sfnam.loki.delve.office.com/stikkontakt.ws/?clientType=OfficeDotCom",
                scheme
            ))
            .body(Body::default())?;
        *req.headers_mut() = gen_header_map();

        let resp = client.request(req).await?;

        println!("status: {}\nheaders: {:?}", resp.status(), resp.headers());
    }

    Ok(())
}

fn gen_header_map() -> HeaderMap {
    let mut map = HeaderMap::new();

    map.insert(SEC_WEBSOCKET_VERSION, HeaderValue::from_static("13"));
    map.insert(
        SEC_WEBSOCKET_KEY,
        HeaderValue::from_static("LvGIQrP7VFE4dgdAs3IZTA=="),
    );
    map.insert(
        SEC_WEBSOCKET_EXTENSIONS,
        HeaderValue::from_static("permessage-deflate; client_max_window_bits"),
    );
    map.insert(CONNECTION, HeaderValue::from_static("Upgrade"));
    map.insert(UPGRADE, HeaderValue::from_static("websocket"));
    // map.insert(
    //     ORIGIN,
    //     HeaderValue::from_str("https://usc-word-edit.officeapps.live.com")?,
    // );
    map.insert(
        HOST,
        HeaderValue::from_static("sfnam.loki.delve.office.com"),
    );
    map.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_static("zip, deflate, br"),
    );
    map.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));

    map
}
