#![allow(non_snake_case)]

use std::borrow::Borrow;

use futures::Future;

use crate::{application::App};
use http_types::{Request, Response};
use crate::application::HANDLERS;

pub struct Context {
	pub handlerIndex: usize,

	pub request: Request,
	pub response: Response,
}

impl Context {
	pub async fn next(mut self) -> Self {
		let handlers = HANDLERS.read().await;
		println!("len={}, index={}", handlers.len(), self.handlerIndex);
		if self.handlerIndex + 1 < handlers.len() {
			println!("Found there is next function");
			self.handlerIndex += 1;
			let handler = handlers.get(self.handlerIndex).unwrap();
			return handler.run(self).await;
		}
		else {
			println!("no next function");
			return self;
		}
	}
}
