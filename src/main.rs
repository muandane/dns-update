use public_ip::addr;
use std::net::IpAddr;
use trust_dns_resolver::TokioAsyncResolver;
use hyper::{Client, Body, Request, Uri};
use hyper_tls::HttpsConnector;



#[tokio::main]
async fn main() {
    let my_ip = addr().await.unwrap();
    
    let resolver = TokioAsyncResolver::tokio_from_system_conf().unwrap();

    // DNS lookup
    let response = resolver.lookup_ip("domain-name.duckdns.org").await.unwrap();

    // Pick the first IP from the response
    let duckdns_ip: IpAddr = response.iter().next().unwrap();

    if my_ip == duckdns_ip {
        println!("IP has not changed.");
    } else {
        println!("IP has changed. Updating DuckDNS.");
        let token = "dns-token";
        let domain = "domain-name";
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let uri = Uri::builder()
            .scheme("https")
            .authority("www.duckdns.org")
            .path_and_query(format!("/update?domains={}&token={}&ip={}", domain, token, my_ip))
            .build()
            .unwrap();
        let req = Request::builder()
            .method("GET")
            .uri(uri.clone())
            .body(Body::empty())
            .unwrap();
        let _response = client.request(req).await.unwrap();
    }
}
