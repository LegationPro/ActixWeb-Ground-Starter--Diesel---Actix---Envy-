use crate::models::*;
use diesel::prelude::*;

pub fn show_posts(connection: &mut PgConnection) {
    use crate::schema::posts::dsl::*;

    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());

    for post in results {
        println!("{}", post.title);
        println!("-------------------\n");
        println!("{}", post.body);
    }
}

pub fn create_post(conn: &mut PgConnection, title: &str, body: &str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}
