#![allow(non_snake_case)]
#![feature(async_closure)]
#![feature(once_cell)]

pub mod application;
pub mod context;
pub mod utils;

#[cfg(test)]
mod tests {
	use crate::{application::App, context::Context};

	#[test]
	fn it_works() {
		let mut app = App::new();
		let _ = app.setHandlerWithPath("/test", hehe);
		let _ = app.setHandlerWithPath("/test.*", doIt);
		let _ = app.start();
	}

	async fn hehe(mut ctx: Context) -> Context {
		println!("Before handle. {:?}", ctx.request.body_string().await);
		ctx.response.set_body("This is hehe function");
		let ctx = ctx.next().await;
		println!("After handler");
		return ctx;
	}

	async fn doIt(mut ctx: Context) -> Context {
		ctx.response.insert_header("Content-Type", "text/plain");
		let mut s = ctx.response.body_string().await.unwrap();
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
