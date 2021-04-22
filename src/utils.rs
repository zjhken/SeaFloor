#![allow(non_snake_case)]

use futures::{future::BoxFuture, Future};
use std::pin::Pin;

use crate::context::Context;

pub struct AsyncFnPtr<R> {
	pub func: Box<dyn Fn(Context) -> BoxFuture<'static, R> + Send + 'static + Sync>,
}

impl<R> AsyncFnPtr<R> {
	pub fn new<F>(f: fn(Context) -> F) -> AsyncFnPtr<F::Output>
		where
				F: Future<Output=R> + Send + 'static,
	{
		AsyncFnPtr {
			func: Box::new(move |a| Box::pin(f(a))),
		}
	}
	pub async fn run(&self, context: Context) -> R {
		return (self.func)(context).await;
	}
}
