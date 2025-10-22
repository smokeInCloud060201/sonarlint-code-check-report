use actix_web::http::Method;
use reqwest::{Client as HttpClient, RequestBuilder};

pub struct Client<RQ, RS> {
    url: String,
    method: Method,
    request: RQ,
    response: Option<RS>,
}

impl<RQ, RS> Client<RQ, RS>
where
    RQ: Into<String> + Clone,
    RS: From<String>
{
    pub fn new(url: String, method: Method, request: RQ) -> Self {
        Self {
            url,
            method,
            request,
            response: None
        }
    }

    pub async fn execute(&self) {

        let client = HttpClient::new();
        let body = self.request.clone().into();

        let request_builder: RequestBuilder = match self.method {
            Method::GET => client.get(&self.url),
            Method::POST => client.post(&self.url).body(body),
            Method::PUT => client.put(&self.url).body(body),
            Method::DELETE => client.delete(&self.url),
            _ => todo!(),
        };

    }
}
