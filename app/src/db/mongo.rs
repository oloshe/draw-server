use actix::{Actor, Context};
use mongodb::{Client, Database};


// async fn init_database() {
// 	let client = Client::with_uri_str("mongodb://localhost:27017")
// 		.await.unwrap();
// 	let database = client.database("dg");

// }

// utils::lazy_static! {
// 	pub static ref DB: Database = create_data();
// }

// impl Actor for DBActor {
//     type Context = Context<Self>;
// }