#![allow(non_snake_case)]

use std::borrow::Borrow;

use anyhow::Result;
use futures::Future;
use http_types::{Request, Response};

use crate::application::{App, PATH_REG};
use crate::application::{HANDLERS, PATHS};

pub struct Context {
	pub pathIndex: usize,

	pub request: Request,
	pub response: Response,
}

impl Context {
	pub async fn next(mut self) -> Result<Self> {
		let handlers = HANDLERS.read().await;
		let pathRegex = PATH_REG.read().await;
		println!("len={}, index={}", handlers.len(), self.pathIndex);

		for i in self.pathIndex..handlers.len() {
			println!("Found there is next function");

			println!("Checking regex");
			let regex = pathRegex.get(i).unwrap();
			let url = self.request.url().as_str();
			if regex.is_match(url) {
				println!("Found matches path: {}", url);
				self.pathIndex += 1;
				let handler = handlers.get(i).unwrap();
				return handler.run(self).await;
			}
		}
		println!("no next function");
		return Ok(self);
	}
}
