#![allow(non_snake_case)]
#![feature(async_closure)]
#![feature(once_cell)]

pub use anyhow;
pub use smol;
pub use http_types;

pub mod application;
pub mod context;
pub mod utils;

#[cfg(test)]
mod tests {
	use anyhow::Result;

	use crate::{application::App, context::Context};

	#[test]
	fn it_works() {
		let mut app = App::new();
		let _ = app.setFunc("/test", hehe);
		let _ = app.setFunc("/test.*", doIt);
		let _ = app.start();
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

	#[test]
	fn split() {
		let x: Vec<&str> = "/haha/hehe/heihei".split('/').collect();
		println!("{:?}", x);
	}
}
