extern crate diesel_demo;

use diesel_demo::*;
fn main() {
    match std::env::var("DATABASE_URL") {
        Ok(ref url) => handle(url),
        Err(_) => println!("get sqlite database path is failed!"),
    }
}

fn handle(url: &str) {
    let conn = get_conn(url);
    match conn {
        Some(ref conn) => {
        	 match insert(conn) {
        		Ok(_) => println!("insert successful"),
        		Err(e) => println!("{:?}", e),
    		}
        },
        None => println!("get sqlite database connection is failed!"),
    }
   
}

fn insert(conn: &SqliteConnection) -> Result<(), String> {
    let post = models::Post {
    	title: "Hello".to_owned(),
    	body: r#"#[macro_use] extern crate diesel;
					extern crate dotenv;
					use diesel::prelude::*;
					use diesel::pg::PgConnection;
					use dotenv::dotenv;
					use std::env;

					pub fn establish_connection() -> PgConnection {
					    dotenv().ok();

					    let database_url = env::var("DATABASE_URL")
					        .expect("DATABASE_URL must be set");
					    PgConnection::establish(&database_url)
					        .expect(&format!("Error connecting to {}", database_url))
					}"#.to_owned()
	};
    let result = diesel::insert(&post)
    	.into(schema::posts::table)
    	.execute(conn);
    match result {
    	  Ok(_) => Ok(()),
    	  Err(e) => Err(format!("{:?}", e)),
    }
}