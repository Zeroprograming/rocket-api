use crate::database::connect_to_db::MongoDB;
use mongodb::{bson, Database};

impl MongoDB {
    pub fn new(database: Database) -> Self {
        MongoDB { database }
    }
}
