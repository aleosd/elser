use elasticsearch::Elasticsearch;


pub struct Elser {
    client: Elasticsearch
}

impl Elser {
    pub fn new(client: Elasticsearch) -> Self {
        Self {
            client: client,
        }
    }

    pub async fn info(&self) {
        let cluster_info = self.client.info();
        let response = cluster_info.send().await.unwrap();
        let content = response.text().await.unwrap();
        println!("{}", content);
    }
}