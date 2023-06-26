use mongodb::{error::Error, Client, Collection};

#[derive(Clone, Debug)]
pub struct Connection {
    pub client: Client,
    database: String,
}

impl Connection {
    async fn test_connection(client: Client, database: String) -> Result<Connection, Error> {
        let request = client.list_databases(None, None).await;

        match request {
            Ok(_) => Ok(Connection { client, database }),
            Err(e) => Err(e),
        }
    }

    pub async fn new(uri: String, database: String) -> Result<Connection, Error> {
        let client = Client::with_uri_str(uri).await;

        match client {
            Ok(client) => Connection::test_connection(client, database).await,
            Err(e) => Err(e),
        }
    }

    pub fn collection<T>(&self, name: &str) -> Collection<T> {
        return self.client.database(&self.database).collection(name);
    }
}
