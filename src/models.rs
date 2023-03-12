use diesel::prelude::*;

use crate::schema::posts;


/*
Using #[derive(Queryable)] assumes that the order of fields on the
Post struct matches the columns in the posts table,
so make sure to define them in the order seen in the schema.rs file.
*/

#[derive(Queryable)] // ---> will generate all of the code needed to load a Post struct from a SQL query.
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}