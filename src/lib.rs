#[macro_use] 
pub extern crate diesel;
#[macro_use] 
pub extern crate diesel_codegen;

pub mod models;
pub mod schema;

pub use diesel::prelude::*;

pub fn get_conn(url: &str) -> Option<SqliteConnection> {
    match SqliteConnection::establish(url) {
        Ok(conn) => Some(conn),
        Err(e) => {
        	println!("{:?}", e);
        	None
        },
    }
}



