#![no_std]
#![cfg_attr(not(target_os = "linux"), no_main)]

extern crate alloc;

use alloc::string::ToString;

use noli::prelude::*;

use net_wasabi::http::HttpClient;

fn main() -> u64 {
    let client = HttpClient::new();
    // INFO: ここで指定している各値は、 `python -m http.server 8080` で立ち上げたサーバに対してのもの
    match client.get("host.test".to_string(), 8080, "/test.html".to_string()) {
        Ok(res) => println!("response:\n{:#?}", res),
        Err(e) => println!("error:\n{:#?}", e),
    };

    0
}

entry_point!(main);
