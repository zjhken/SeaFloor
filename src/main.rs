use anyhow::bail;
use anyhow::Result;
use regex::Error;

use seafloor::{application::App, context::Context};

fn main() -> Result<()> {
    App::new()
        .setFunc("/test", hehe)
        .setFunc("/test.*", doIt)
        .start()
}

async fn hehe(mut ctx: Context) -> Result<Context, http_types::Error> {
    println!("Enter hehe");
    let s = ctx.request.body_string().await?;
    ctx.response.set_body("This is hehe function");
    ctx.sessionData.insert("haha", Box::new(s));
    let ctx = ctx.next().await;
    println!("hehe done");
    return ctx;
}

async fn doIt(mut ctx: Context) -> Result<Context, http_types::Error> {
    println!("Enter doIt");
    ctx.response.insert_header("Content-Type", "text/plain");
    let s = ctx.sessionData.get("haha").unwrap().to_string();
    // s.push_str("This is doIt function");
    ctx.response.set_body(s);
    let ctx = ctx.next().await;
    println!("DoIt done.");
    return ctx;
}
