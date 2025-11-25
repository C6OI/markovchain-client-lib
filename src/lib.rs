#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

use reqwest::{Client, IntoUrl, Response, Url};
use serde::Serialize;
use url::ParseError;
use crate::content_string::ContentString;
use crate::error::Error;

pub mod content_string;
mod error;

pub struct MarkovChainClient {
    addr: Url,
    client: Client
}

#[derive(Debug, Serialize)]
pub struct InputPayload {
    pub input: ContentString
}

#[derive(Debug, Serialize)]
pub struct GeneratePayload {
    pub start: Option<ContentString>,
    pub max_length: Option<usize>
}

impl MarkovChainClient {
    /// Initialize a new client for the markov chain API
    /// 
    /// # Panics
    /// Will panic if failed to convert the `addr`
    #[must_use]
    pub fn new<U: IntoUrl>(addr: U) -> Self {
        Self {
            addr: addr.into_url().expect("Failed to convert addr"),
            client: Client::new(),
        }
    }

    /// Save the text in the database
    ///
    /// # Errors
    /// Will return `Err` if an error occurred while serializing the payload or sending a request
    /// to the server.
    ///
    /// # Panics
    /// Will panic if the function cannot parse an endpoint URL
    pub async fn input(&self, payload: InputPayload) -> Result<(), Error> {
        let endpoint = self.get_url("generate").expect("Failed to get url");
        let payload = Self::serialize(&payload)?;

        self.post(endpoint, payload).await?;
        Ok(())
    }

    /// Generate new text
    ///
    /// # Errors
    /// Will return `Err` if an error occurred while serializing the payload, sending a request
    /// to the server or reading the response body.
    ///
    /// # Panics
    /// Will panic if the function cannot parse an endpoint URL
    pub async fn generate(&self, payload: GeneratePayload) -> Result<String, Error> {
        let endpoint = self.get_url("generate").expect("Failed to get url");
        let payload = Self::serialize(&payload)?;

        let response = self.post(endpoint, payload).await?;
        let text = response.text().await?;

        Ok(text)
    }

    async fn post<U: IntoUrl>(&self, endpoint: U, payload: String) -> Result<Response, Error> {
        let response = self.client
            .post(endpoint)
            .body(payload)
            .send()
            .await?;

        let status = response.status();

        if status.is_success() {
            Ok(response)
        } else {
            Err(Error::Api {
                status,
                body: response.text().await?
            })
        }
    }

    fn serialize<T: ?Sized + Serialize>(value: &T) -> Result<String, Error> {
        serde_json::to_string(&value).map_err(Error::Json)
    }

    fn get_url(&self, endpoint: &str) -> Result<Url, ParseError> {
        self.addr.join(endpoint)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};
    use super::*;

    #[test]
    fn input_payload() {
        let json = json!({
            "input": "qweasd123"
        });

        let payload = InputPayload { input: "qweasd123".try_into().unwrap() };

        let from_payload = serde_json::to_string(&payload).unwrap();
        let from_payload: Value = serde_json::from_str(&from_payload).unwrap();

        assert_eq!(json, from_payload);
    }

    #[test]
    fn generate_payload() {
        let assert = |json: &Value, payload: &GeneratePayload| {
            let from_payload = serde_json::to_string(payload).unwrap();
            let from_payload: Value = serde_json::from_str(&from_payload).unwrap();

            assert_eq!(*json, from_payload);
        };

        let mut json = json!({
            "start": null,
            "max_length": null
        });

        let mut payload = GeneratePayload {
            start: None,
            max_length: None
        };

        assert(&json, &payload);

        json["start"] = "qweasd123".into();
        payload.start = Some("qweasd123".try_into().unwrap());
        assert(&json, &payload);

        json["max_length"] = 42.into();
        payload.max_length = Some(42);
        assert(&json, &payload);
    }
}
