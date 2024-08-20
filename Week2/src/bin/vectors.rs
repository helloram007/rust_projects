fn main() {
    
    // Two ways to create Vectors
    // 1. Vec object using new()
    // 2. macro vec![1,2,3]

    // First Way using new
    let mut v: Vec<String> = Vec::new();
    v.push(String::from("One"));
    v.push(String::from("Two"));
    v.push(String::from("Three"));

    // Using macro vec![1,2,3]
    let v2 = vec![1,2,3];

    // Two ways to access vector elements
    // 1. brackets way e.g &v[1]
    // 2. using get method on vec &v.get(1)

    // using bracket to access first element
    let s = &v[0];

    // using get Method to access first element
    let s = v.get(0);

    if let Some(e) = s {
        println!("{e}");
    }

    for s in &mut v {
        s.push_str("!");
    }

    for s in &v {
        println!("{s}");
    }

    // create a empty vector
    let mut v3: Vec<String> = vec![];

    for s in v.into_iter() {
        v3.push(s);
    }

    //let i = v.get(0);
    

} // v is dropped