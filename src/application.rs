#![allow(non_snake_case)]

use std::net::{SocketAddr, TcpListener};

use anyhow::Error;
use anyhow::Result;
use smol::{future::Future, Async};

use crate::{context::Context, utils::AsyncFnPtr};
use futures::AsyncReadExt;
use http_types::{Request, Response, StatusCode};
use smol::lock::RwLock;
use std::collections::HashMap;
use std::lazy::SyncLazy;
use smol::io::AsyncWriteExt;
use regex::Regex;

pub static HANDLERS: SyncLazy<RwLock<Vec<AsyncFnPtr<Context>>>> =
	SyncLazy::new(|| RwLock::new(vec![]));
pub static PATHS: SyncLazy<RwLock<Vec<&str>>> = SyncLazy::new(|| RwLock::new(vec![]));
pub static PATH_TREE: SyncLazy<RwLock<HashMap<&'static str, PathNode>>> =
	SyncLazy::new(|| RwLock::new(HashMap::new()));

pub static PATH_REG: SyncLazy<RwLock<Vec<Regex>>> = SyncLazy::new(|| RwLock::new(vec![]));

pub struct App {
	addr: ([u8; 4], u16),
}

pub enum PathNode {
	Int(usize),
	Str(&'static str),
	Vec(Vec<PathNode>),
}

impl App {
	pub fn setHandlerWithPath<Fut>(&mut self, path: &'static str, f: fn(Context) -> Fut) -> &mut App
		where
				Fut: Future<Output=Context> + Send + 'static,
	{
		smol::block_on(async {
			if !path.starts_with('/') {
				panic!("Path should start with '/', now is {}", path);
			}

			let mut handlers = HANDLERS.write().await;
			(*handlers).push(AsyncFnPtr::new(f));

			// let index = handlers.len() - 1;

			// let mut paths = PATHS.write().await;
			// (*paths).push(path);

			let mut pathRegex = PATH_REG.write().await;
			let regex = Regex::new(path).unwrap();
			pathRegex.push(regex);
		});
		return self;
	}

	pub fn new() -> Self {
		return App {
			addr: ([0, 0, 0, 0], 8800),
		};
	}

	pub fn listenAddress<A: Into<SocketAddr>>(&mut self, addr: ([u8; 4], u16)) -> &mut App {
		self.addr = addr;
		return self;
	}

	pub fn start(&mut self) -> Result<()> {
		let result: Result<(), std::io::Error> = smol::block_on(async {
			let listener = Async::<TcpListener>::bind(self.addr).unwrap();
			println!("Listening on {}", listener.get_ref().local_addr()?);
			println!("Now start a TCP client.");
			loop {
				let (stream, peer_addr) = listener.accept().await?;
				println!("Accepted client: {}", peer_addr);

				let stream = async_dup::Arc::new(stream);

				// Spawn a task that echoes messages from the client back to it.
				smol::spawn(async move {
					if let Err(err) = async_h1::accept(stream, async move |req| {
						println!("Serving {}", req.url());

						let mut ctx = Context {
							pathIndex: 0,
							request: req,
							response: Response::new(StatusCode::Ok),
						};

						let handlers = HANDLERS.read().await;
						if handlers.len() != 0 {
							let handler = handlers.get(0).unwrap();
							let ctx = handler.run(ctx).await;
							return Ok(ctx.response);
						} else {
							return Err(http_types::Error::new(
								StatusCode::ServiceUnavailable,
								Error::msg("not work"),
							));
						}
					})
							.await
					{
						println!("Connection error: {:#?}", err);
					}
				})
						.detach();
			}
			Ok(())
		});
		return Ok(());
	}
}
