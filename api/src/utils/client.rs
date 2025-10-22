use reqwest::{Client as HttpClient, Method, RequestBuilder};
use serde::de::DeserializeOwned;

pub struct Client<RQ, RS> {
    url: String,
    method: Method,
    username: Option<String>,
    password: Option<String>,
    request: RQ,
    response_type: RS,
}

impl<RQ, RS> Client<RQ, RS>
where
    RQ: Into<String> + Clone,
    RS: DeserializeOwned
{
    pub fn new(url: String,
               method: Method,
               request: RQ,
               response_type: RS,
               username: Option<String>,
               password: Option<String>
    ) -> Self {
        Self {
            url,
            method,
            request,
            response_type,
            username,
            password
        }
    }

    pub async fn execute(&self) -> anyhow::Result<RS> {

        let client = HttpClient::new();
        let body = self.request.clone().into();

        let mut request_builder: RequestBuilder = match self.method {
            Method::GET => client.get(&self.url),
            Method::POST => client.post(&self.url).body(body),
            Method::PUT => client.put(&self.url).body(body),
            Method::DELETE => client.delete(&self.url),
            _ => client.request(self.method.clone(), &self.url),
        };

        request_builder = request_builder.basic_auth(self.username.clone().unwrap_or_default(), Some(self.password.clone().unwrap_or_default()));
        let response = request_builder.send().await?;
        let result = response.json::<RS>().await?;

        Ok(result)
    }
}
