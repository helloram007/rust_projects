struct BrowserCommand<T> {
    name: String,
    payload: T,
}

impl<T> BrowserCommand<T> {
    fn new(name: String, payload: T) -> self {
        BrowserCommand{
            name,
            payload,
        }
    }
}

fn main() {
    let cmd1 = BrowserCommand::new{
        "navigate".to_owned(),
        "https://letsgetrusty.com".to_owned(),
    }
    let cmd2 = BrowserCommand::new{
        "zoom".to_owned(),
        200,
    }
    
}