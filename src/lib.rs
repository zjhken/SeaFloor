#![allow(non_snake_case)]
#![feature(async_closure)]
#![feature(once_cell)]

mod application;
mod context;
mod utils;

#[cfg(test)]
mod tests {
	use crate::{application::App, context::Context};

	#[test]
	fn it_works() {
		let mut app = App::new();
		let _ = app.setHandler(hehe);
		let _ = app.setHandler(doIt);
		let _ = app.start();
	}

	async fn hehe(mut ctx: Context) -> Context {
		println!("Before handle. {:?}", ctx.request.body_string().await);
		let ctx = ctx.next().await;
		println!("After handler");
		return ctx;
	}

	async fn doIt(mut ctx: Context) -> Context {
		ctx.response.insert_header("Content-Type", "text/plain");
		ctx.response.set_body("Hello from async-h1!");
		let ctx = ctx.next().await;
		println!("DoIt done.");
		return ctx;
	}
}
