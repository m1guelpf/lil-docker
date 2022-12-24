use reqwest::Client;

use crate::image::Image;

pub struct Hub {
    client: Client,
    token: Option<String>,
}

impl Hub {
    pub async fn pull(image: &str) -> Image {
        let mut hub = Self {
            token: None,
            client: Client::new(),
        };

        let mut image = Image::new(image);

        let res = hub.client
            .get(&format!("https://auth.docker.io/token?service=registry.docker.io&scope=repository:library/{}:pull", image.name))
            .send()
            .await
            .unwrap()
            .json::<serde_json::Value>()
            .await
            .unwrap();

        hub.token = Some(res["token"].as_str().unwrap().to_string());

        image.get_layers(&hub).await;

        image
    }

    pub async fn get_manifest(&self, name: &String, revision: &String) -> serde_json::Value {
        self.query(format!(
            "https://registry.hub.docker.com/v2/library/{name}/manifests/{revision}"
        ))
        .await
        .json::<serde_json::Value>()
        .await
        .unwrap()
    }

    pub async fn get_layer(&self, name: &String, layer: &str) -> bytes::Bytes {
        self.query(format!(
            "https://registry.hub.docker.com/v2/library/{name}/blobs/{layer}"
        ))
        .await
        .bytes()
        .await
        .unwrap()
    }

    async fn query(&self, url: String) -> reqwest::Response {
        self.client
            .get(&url)
            .bearer_auth(self.token.as_ref().unwrap())
            .send()
            .await
            .unwrap()
    }
}
