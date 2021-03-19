#![allow(non_snake_case)]

use anyhow::Result;
use anyhow::Error;
use smol::future::Future;
use std::pin::Pin;
use futures::future::BoxFuture;

pub struct AsyncFnPtr<R> {
	func: Box<dyn Fn(i32, i32) -> BoxFuture<'static, R> + Send + 'static>
}

impl<R> AsyncFnPtr<R> {
	pub fn new<F>(f: fn(i32, i32) -> F) -> AsyncFnPtr<F::Output>
		where F: Future<Output=R> + Send + 'static {
		AsyncFnPtr {
			func: Box::new(move |a, b| Box::pin(f(a, b))),
		}
	}
	// async fn run(&self) -> R { (self.func)().await }
}


pub struct App {
	handlers: Vec<Box<dyn Future<Output=bool>>>,
}

impl App {
	pub async fn setHandler<Fut>(mut handlers: Vec<AsyncFnPtr<bool>>, f: fn(i32, i32) -> Fut) -> Result<bool, Error>
		where
				Fut: Future<Output=bool> + Send + 'static,
	{
		handlers.push(AsyncFnPtr::new(f));
		handlers
				.iter()
				.enumerate()
				.map(|(index, asyncFnPtr)| {
					smol::block_on(async {
						(asyncFnPtr.func)(index as i32, 1i32).await
						// asyncFnPtr(index as i32, 1i32).await
					})
				})
				.collect::<Vec<bool>>();
		// Ok(f(1, 2).await)
		Ok(true)
	}
}

pub fn next(){

}