use mongodb::{Client, Collection};

#[derive(Clone)]
pub struct Connection {
    client: Client,
    database: String,
}

impl Connection {
    pub async fn new(uri: String, database: String) -> Self {
        let client = Client::with_uri_str(uri).await.expect("failed to connect");
        Connection { client, database }
    }

    pub fn collection<T>(&self, name: &str) -> Collection<T> {
        return self.client.database(&self.database).collection(name);
    }
}
