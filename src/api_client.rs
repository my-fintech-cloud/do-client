use hyper::{Body, Client, Method, Request, Response};
use hyper_tls::HttpsConnector;
use serde::{de::DeserializeOwned, Serialize};

use crate::{models::CreateNodePoolRequest, NodePullResponse};

pub struct DoApiClient {
    pub token: String,
}

impl DoApiClient {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    pub async fn get_node_pool(
        &self,
        cluster_id: String,
        node_pool_id: String,
    ) -> NodePullResponse {
        let url = format!(
            "https://api.digitalocean.com/v2/kubernetes/clusters/{}/node_pools/{}",
            cluster_id, node_pool_id
        );

        let request: Option<&String> = None;

        return self.execute_request(url, Method::GET, request).await;
    }

    pub async fn add_node_pool(
        &self,
        cluster_id: String,
        request: CreateNodePoolRequest,
    ) -> NodePullResponse {
        let url = format!(
            "https://api.digitalocean.com/v2/kubernetes/clusters/{}/node_pools",
            cluster_id
        );

        return self
            .execute_request(url, Method::POST, Some(&request))
            .await;
    }

    pub async fn remove_node_pool(&self, cluster_id: String, node_pool_id: String) {
        let url = format!(
            "https://api.digitalocean.com/v2/kubernetes/clusters/{}/node_pools/{}",
            cluster_id, node_pool_id
        );

        let request: Option<&String> = None;
        self.execute_request_without_response(url, Method::DELETE, request)
            .await;
    }

    async fn execute_request<T: DeserializeOwned>(
        &self,
        url: String,
        method: Method,
        request: Option<&impl Serialize>,
    ) -> T {
        let req = Request::builder()
            .method(method)
            .uri(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json");

        let body = match request {
            Some(src) => Body::from(serde_json::to_string(src).unwrap()),
            None => Body::empty(),
        };

        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let req = req.body(body).unwrap();
        let response = client.request(req).await.unwrap();
        let body = get_body(response).await;

        println!("{}", String::from_utf8(body.clone()).unwrap());

        return serde_json::from_slice(&body[..]).unwrap();
    }

    async fn execute_request_without_response(
        &self,
        url: String,
        method: Method,
        request: Option<&impl Serialize>,
    ) {
        let req = Request::builder()
            .method(method)
            .uri(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json");

        let body = match request {
            Some(src) => Body::from(serde_json::to_string(src).unwrap()),
            None => Body::empty(),
        };

        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let req = req.body(body).unwrap();
        client.request(req).await.unwrap();
    }
}

async fn get_body(response: Response<Body>) -> Vec<u8> {
    let body = response.into_body();
    let full_body = hyper::body::to_bytes(body).await.unwrap();
    full_body.iter().cloned().collect::<Vec<u8>>()
}
