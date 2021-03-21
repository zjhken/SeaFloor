#![allow(non_snake_case)]

use std::net::{SocketAddr, TcpListener};

use anyhow::Error;
use anyhow::Result;
use smol::{Async, future::Future};

use crate::{context::Context, utils::AsyncFnPtr};
use futures::AsyncReadExt;
use http_types::{Response, Request, StatusCode};
use smol::lock::{RwLock};
use std::lazy::SyncLazy;


pub static HANDLERS: SyncLazy<RwLock<Vec<AsyncFnPtr<Context>>>> = SyncLazy::new(||RwLock::new(vec![]));

pub struct App {
	addr: ([u8; 4], u16),
}

impl App {
	pub fn setHandler<Fut>(&mut self, f: fn(Context) -> Fut) -> &mut App
		where
				Fut: Future<Output=Context> + Send + 'static,
	{
		smol::block_on(async {
			let mut handlers = HANDLERS.write().await;
			(*handlers).push(AsyncFnPtr::new(f));
		});
		return self;
	}

	pub fn new() -> Self {
		return App {
			addr: ([0, 0, 0, 0], 8800),
		};
	}

	pub fn listenAddress<A: Into<SocketAddr>>(&mut self, addr: ([u8;4], u16)) -> &mut App {
		self.addr = addr;
		return self;
	}

	pub fn start(&mut self) -> Result<()> {
		let result: Result<(), std::io::Error> = smol::block_on(async {
			let listener = Async::<TcpListener>::bind(self.addr).unwrap();
			println!("Listening on {}", listener.get_ref().local_addr()?);
			println!("Now start a TCP client.");
			loop {
				let (mut stream, peer_addr) = listener.accept().await?;
				println!("Accepted client: {}", peer_addr);

				let stream = async_dup::Arc::new(stream);

				// Spawn a task that echoes messages from the client back to it.
				smol::spawn(async move {

					if let Err(err) = async_h1::accept(stream, async move |req|{
						println!("Serving {}", req.url());

						let mut ctx = Context{
							handlerIndex: 0,
							request: req,
							response: Response::new(StatusCode::Ok),
						};

						let handlers = HANDLERS.read().await;
						if handlers.len() != 0 {
							let handler = handlers.get(0).unwrap();
							let ctx = handler.run(ctx).await;
							return Ok(ctx.response);
						}
						else {
							return Err(http_types::Error::new(StatusCode::ServiceUnavailable, Error::msg("not work")));
						}
					}).await {
						println!("Connection error: {:#?}", err);
					}
				}).detach();
			}
			Ok(())
		});
		return Ok(());
	}
}


async fn serve(req: Request) -> http_types::Result<Response> {
	println!("Serving {}", req.url());

	let mut res = Response::new(StatusCode::Ok);
	res.insert_header("Content-Type", "text/plain");
	res.set_body("Hello from async-h1!");
	Ok(res)
}
