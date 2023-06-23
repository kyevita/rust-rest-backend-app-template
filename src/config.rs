use envy;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Configuration {
    pub db_conn_str: String,
    pub db_name: String,
    pub users_collection: String,
}

pub fn load_config() -> Configuration {
    return envy::from_env::<Configuration>().expect("Invalid env file");
}
