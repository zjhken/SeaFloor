#![allow(non_snake_case)]

use anyhow::Result;
use http_types::{Request, Response};

use crate::application::{PATH_REG};
use crate::application::{HANDLERS};
use std::collections::HashMap;
use std::fmt::Display;
use async_dup::Arc;
use smol::lock::RwLock;

pub struct Context {
	pub pathIndex: usize,

	pub request: Request,
	pub response: Response,
	pub sessionData: HashMap<&'static str, Box<dyn Display + Send>>
}


impl Context {
	pub async fn next(mut self) -> Result<Self, http_types::Error> {
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
