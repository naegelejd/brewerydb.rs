extern crate hyper;
extern crate rustc_serialize;

use std::io::Read;

use hyper::header::Connection;

use rustc_serialize::json;

pub struct Client {
    api_key: String,
    client: hyper::Client,
}

#[derive(RustcDecodable)]
pub struct Response {
    message: String,
    status: String,
}

impl Client {
    pub fn new(api_key: &str) -> Client {
        Client{api_key: format!("{}", api_key), client: hyper::Client::new()}
    }

    pub fn heartbeat(&mut self) -> Result<Response, &'static str> {
        let url = format!("http://api.brewerydb.com/v2/heartbeat?key={}", self.api_key);
        let mut res = self.client.get(&url)
            .header(Connection::close())
            .send().unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        let decoded: Response = json::decode(&body).unwrap();
        Ok(decoded)
    }
}

#[test]
fn it_works() {
    let mut c = Client::new("870d6d3a12618dd25a17bf374aa0062c");
    let resp = c.heartbeat().unwrap();
    assert_eq!(resp.message, "Request Successful");
    assert_eq!(resp.status, "success");
}
