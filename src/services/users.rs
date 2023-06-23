use mongodb::{bson::doc, error::Error, Collection};

use crate::{
    config::Configuration,
    db::{connection::Connection, models::user::User},
};

pub struct UserService {
    connection: Connection,
    collection: Collection<User>,
    configuration: Configuration,
}

impl UserService {
    pub fn new(connection: Connection, configuration: Configuration) -> Self {
        let collection_name = configuration.users_collection.clone();
        let collection = connection.collection(&collection_name);

        UserService {
            connection,
            collection,
            configuration,
        }
    }

    pub async fn get_user_by_id(&self, id: &str) -> Result<Option<User>, Error> {
        return self
            .collection
            .find_one(
                doc! {
                  "_id": id
                },
                None,
            )
            .await;
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, Error> {
        return self
            .collection
            .find_one(
                doc! {
                  "username": username
                },
                None,
            )
            .await;
    }

    pub async fn create_user(&self, body: User) -> Result<Option<User>, Error> {
        let result = self.collection.insert_one(body, None).await;

        match result {
            Ok(insert_result) => {
                self.get_user_by_id(&insert_result.inserted_id.to_string())
                    .await
            }
            Err(err) => Err(err),
        }
    }
}
