use schema::*;

#[derive(Debug, Insertable)]
#[table_name="posts"]
pub struct Post {
    pub title: String,
    pub body: String,
}

// impl_Insertable! {
// 	(posts)
// 	struct Post {
//     	title: String,
//     	body: String,
// 	}
// }