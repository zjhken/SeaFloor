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
	use anyhow::bail;
	use anyhow::Result;
	use regex::Error;

	use crate::{application::App, context::Context};

	#[test]
	fn it_works() {
		let mut app = App::new();
		let _ = app.setFunc("/test", hehe);
		let _ = app.setFunc("/test.*", doIt);
		let _ = app.start();
	}

	async fn hehe(mut ctx: Context) -> Result<Context, http_types::Error> {
		let s = ctx.request.body_string().await?;
		ctx.response.set_body("This is hehe function");
		let ctx = ctx.next().await;
		println!("After handler");
		return ctx;
	}

	async fn doIt(mut ctx: Context) -> Result<Context, http_types::Error> {
		ctx.response.insert_header("Content-Type", "text/plain");
		let mut s = ctx.response.body_string().await?;
		s.push_str("This is doIt function");
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
