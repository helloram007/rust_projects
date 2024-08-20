fn main() {
    let username = get_username(1);
    match &username {
        Some(username) => println!("using match  expr: {username}"),
        None => {}
    }
    if let Some(name) = &username {
        println!("using let : {name}");
    }
}

fn get_username(user_id: u32) -> Option<String> {
    //get username from database
    let db_result = String::from("Ferris");
    if user_id == 1 {
        Some(db_result)
    } else {
        None
    }
}
