use blog_shared::Post;

fn main() {
    let post = Post::new(
        "Post on the Web".to_owned(),
        "Let's get Rusty!".to_owned(),
    );
    println!("{post:?}");
}
 