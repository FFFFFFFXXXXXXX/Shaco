use reqwest::{
    header::{HeaderMap, HeaderValue},
    Certificate,
};
use riot_local_auth::Credentials;
use serde::Serialize;

/// A client for the League-Client(LCU) REST API
pub struct RESTClient {
    port: String,
    reqwest_client: reqwest::Client,
}

impl RESTClient {
    pub fn new() -> riot_local_auth::Result<Self> {
        Ok(Self::new_internal(
            &riot_local_auth::lcu::try_get_credentials()?,
        ))
    }

    /// Create a new instance of the LCU REST wrapper
    fn new_internal(credentials: &Credentials) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&credentials.basic_auth()).unwrap(),
        );

        let reqwest_client = reqwest::ClientBuilder::new()
            .add_root_certificate(
                Certificate::from_pem(include_bytes!("../riotgames.pem")).unwrap(),
            )
            .default_headers(headers)
            .build()
            .unwrap();

        Self {
            port: credentials.port.to_string(),
            reqwest_client,
        }
    }

    /// Make a get request to the specified endpoint
    pub async fn get(&self, endpoint: &str) -> Result<serde_json::Value, reqwest::Error> {
        self.reqwest_client
            .get(format!("https://127.0.0.1:{}{}", self.port, endpoint))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
            .or_else(|_| Ok(serde_json::Value::Null))
    }

    /// Make a post request to the specified endpoint
    pub async fn post<T: Serialize>(
        &self,
        endpoint: &str,
        body: T,
    ) -> Result<serde_json::Value, reqwest::Error> {
        self.reqwest_client
            .post(format!("https://127.0.0.1:{}{}", self.port, endpoint))
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
            .or_else(|_| Ok(serde_json::Value::Null))
    }

    /// Make a put request to the specified endpoint
    pub async fn put<T: Serialize>(
        &self,
        endpoint: &str,
        body: T,
    ) -> Result<serde_json::Value, reqwest::Error> {
        self.reqwest_client
            .put(format!("https://127.0.0.1:{}{}", self.port, endpoint))
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
            .or_else(|_| Ok(serde_json::Value::Null))
    }

    /// Make a delete request to the specified endpoint
    pub async fn delete(&self, endpoint: &str) -> Result<serde_json::Value, reqwest::Error> {
        self.reqwest_client
            .delete(format!("https://127.0.0.1:{}{}", self.port, endpoint))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
            .or_else(|_| Ok(serde_json::Value::Null))
    }

    /// Make a patch request to the specified endpoint
    pub async fn patch<T: Serialize>(
        &self,
        endpoint: &str,
        body: T,
    ) -> Result<serde_json::Value, reqwest::Error> {
        self.reqwest_client
            .patch(format!("https://127.0.0.1:{}{}", self.port, endpoint))
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
            .or_else(|_| Ok(serde_json::Value::Null))
    }
}

impl From<&Credentials> for RESTClient {
    fn from(credentials: &Credentials) -> Self {
        Self::new_internal(credentials)
    }
}
