use anyhow::Result;

use seafloor::{application::App, context::Context};

fn main() -> Result<()> {
    App::new()
        .setFunc("/test", hehe)
        .setFunc("/test.*", do_it)
        .listenAddress(([0,0,0,0], 8800))
        .start()
}

async fn hehe(mut ctx: Context) -> Result<Context, http_types::Error> {
    println!("Enter hehe");
    ctx.response.set_body("This is hehe function");
    ctx.sessionData.insert("user", Box::new(User{name: "Tom".to_string(), age: 3}));
    let ctx = ctx.next().await;
    println!("hehe done");
    return ctx;
}

async fn do_it(mut ctx: Context) -> Result<Context, http_types::Error> {
    println!("Enter doIt");
    ctx.response.insert_header("Content-Type", "text/plain");
    let s = ctx.sessionData.get("user").unwrap().to_string();
    // s.push_str("This is doIt function");
    ctx.response.set_body(s);
    let ctx = ctx.next().await;
    println!("DoIt done.");
    return ctx;
}


struct User {
    name: String,
    age: u8,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}