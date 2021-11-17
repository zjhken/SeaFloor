#![allow(non_snake_case)]

use std::collections::HashMap;
use std::lazy::SyncLazy;
use std::net::{SocketAddr, TcpListener};

use anyhow::Result;
use http_types::{Response, StatusCode};
use regex::Regex;
use smol::{Async, future::Future};
use smol::lock::RwLock;

use crate::{context::Context, utils::AsyncFnPtr};

pub static HANDLERS: SyncLazy<RwLock<Vec<AsyncFnPtr<HttpResult>>>> =
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

pub type HttpResult = Result<Context, http_types::Error>;

impl App {
	pub fn setFunc<Fut>(&mut self, path: &'static str, f: fn(Context) -> Fut) -> &mut App
		where
				Fut: Future<Output=HttpResult> + Send + 'static,
	{
		smol::block_on(async {
			let mut handlers = HANDLERS.write().await;
			(*handlers).push(AsyncFnPtr::new(f));


			let mut pathRegex = PATH_REG.write().await;
			let regex = Regex::new(path).unwrap();
			pathRegex.push(regex);
		});
		return self;
	}

	pub fn new() -> Self {
		App {
			addr: ([0, 0, 0, 0], 8800),
		}
	}

	pub fn listenAddress(&mut self, addr: ([u8; 4], u16)) -> &mut App {
		self.addr = addr;
		return self;
	}

	pub fn start(&mut self) -> Result<()> {
		let _: Result<(), std::io::Error> = smol::block_on(async {
			let listener = Async::<TcpListener>::bind(self.addr).unwrap();
			println!("Listening on {}", listener.get_ref().local_addr()?);
			loop {
				let (stream, peer_addr) = listener.accept().await?;
				println!("Accepted client: {}", peer_addr);

				let stream = async_dup::Arc::new(stream);

				// Spawn a task that echoes messages from the client back to it.
				smol::spawn(async move {
					if let Err(err) = async_h1::accept(stream, async move |req| {
						println!("Serving {}", req.url());

						let ctx = Context {
							pathIndex: 0,
							request: req,
							response: Response::new(StatusCode::Ok),
							sessionData: Default::default()
						};

						let handlers = HANDLERS.read().await;
						if handlers.len() == 0 {
							let resp = Response::new(StatusCode::ServiceUnavailable);
							return Ok(resp);
						} else {
							return match ctx.next().await {
								Ok(ctx) => Ok(ctx.response),
								Err(e) => {
									let mut resp = Response::new(e.status());
									resp.set_body(format!("{}", e));
									Ok(resp)
								}
							};
						}
					})
							.await
					{
						println!("Connection error: {:#?}", err);
					}
				})
						.detach();
			}
		});
		return Ok(());
	}
}

impl Default for App {
	fn default() -> Self {
		Self::new()
	}
}
