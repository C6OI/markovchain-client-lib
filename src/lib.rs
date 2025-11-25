use reqwest::{Client, Response, Url};
use serde::Serialize;
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
    input: ContentString
}

#[derive(Debug, Serialize)]
pub struct GeneratePayload {
    start: Option<ContentString>,
    max_length: Option<usize>
}

impl MarkovChainClient {
    /// Initialize a new client for the markov chain API
    pub fn new(addr: Url) -> Self {
        Self {
            addr,
            client: Client::new(),
        }
    }

    /// Save the text in the database
    pub async fn input(&self, payload: InputPayload) -> Result<(), Error> {
        let endpoint = format!("{}/input", self.addr);
        let payload = Self::serialize(&payload)?;

        self.post(endpoint, payload).await?;
        Ok(())
    }

    /// Generate new text
    pub async fn generate(&self, payload: GeneratePayload) -> Result<String, Error> {
        let endpoint = format!("{}/generate", self.addr);
        let payload = Self::serialize(&payload)?;

        let response = self.post(endpoint, payload).await?;
        let text = response.text().await?;

        Ok(text)
    }

    async fn post(&self, endpoint: String, payload: String) -> Result<Response, Error> {
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
        serde_json::to_string(&value).map_err(|e| Error::Json(e))
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

        let payload = InputPayload { input: "qweasd123".into() };

        let from_payload = serde_json::to_string(&payload).unwrap();
        let from_payload: Value = serde_json::from_str(&from_payload).unwrap();

        assert_eq!(json, from_payload);
    }

    #[test]
    fn generate_payload() {
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
        payload.start = Some("qweasd123".into());
        assert(&json, &payload);

        json["max_length"] = 42.into();
        payload.max_length = Some(42);
        assert(&json, &payload);

        fn assert(json: &Value, payload: &GeneratePayload) {
            let from_payload = serde_json::to_string(payload).unwrap();
            let from_payload: Value = serde_json::from_str(&from_payload).unwrap();

            assert_eq!(*json, from_payload);
        }
    }
}
