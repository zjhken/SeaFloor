#![allow(non_snake_case)]
#![feature(async_closure)]
#![feature(once_cell)]
#![feature(associated_type_defaults)]

pub use anyhow;
pub use http_types;
pub use smol;

pub mod application;
pub mod context;
pub mod utils;
pub mod logger;

#[cfg(test)]
mod tests {
	use std::mem::ManuallyDrop;

use anyhow::Result;
use futures::FutureExt;

	use crate::{application::App, context::Context};

	#[test]
	fn it_works() {
		let mut app = App::new();
		let _ = app.setFunc("/test", |ctx| async move {hehe(ctx).await}.boxed());
		let _ = app.setFunc("/test.*", |ctx|async move {doIt(ctx).await}.boxed());
		let _ = app.start();
	}

	async fn hehe(ctx: &mut Context) -> Result<(), http_types::Error> {
		println!("Enter hehe");
		let s = ctx.request.body_string().await?;
		ctx.response.set_body("This is hehe function");
		ctx.sessionData.insert("haha", Box::new(s));
		// let ctx = ctx.next().await;
		println!("hehe done");
		return Ok(());
	}

	async fn doIt(ctx: &mut Context) -> Result<(), http_types::Error> {
		println!("Enter doIt");
		ctx.response.insert_header("Content-Type", "text/plain");
		let s = ctx.sessionData.get("haha").unwrap().to_string();
		// s.push_str("This is doIt function");
		ctx.response.set_body(s);
		// let ctx = ctx.next().await;
		println!("DoIt done.");
		return Ok(());
	}

	#[test]
	fn split() {
		let x: Vec<&str> = "/haha/hehe/heihei".split('/').collect();
		println!("{:?}", x);
	}
}
