use actix::{Actor, Context};
use mongodb::{Client, Database};


pub struct MongoActor {
	client: Client,
	database: Database,
}

impl MongoActor {
	pub async fn new() -> Self {
		let client = Client::with_uri_str("mongodb://localhost:27017")
			.await.unwrap();
		let database = client.database("dg");
		Self {
			client,
			database,
		}
	}
}

impl Actor for MongoActor {
    type Context = Context<Self>;
}