#![allow(non_snake_case)]

use futures::future::{BoxFuture};
use std::net::{TcpListener};
use std::vec;

use anyhow::Result;
use http_types::{Response, StatusCode};
use regex::Regex;
use smol::{future::Future, Async};

use crate::logger::setup_logger;
use crate::{context::Context};

pub struct App {
	addr: ([u8; 4], u16),
	routes: Vec<Route>,
}

#[derive(Clone)]
pub struct Route {
	pub regex: Regex,
	pub func: fn(&mut Context) -> BoxFuture<'_, HttpResult>,
}

pub enum PathNode {
	Int(usize),
	Str(&'static str),
	Vec(Vec<PathNode>),
}

pub type HttpResult = Result<(), http_types::Error>;

pub trait AsyncFn<'a, Out>: Fn(&'a mut Context) -> Self::Fut {
    type Fut: Future<Output = Out> + 'a + Send;
}

impl<'a, F, Out, Fut> AsyncFn<'a, Out> for F
where
    F: Fn(&'a mut Context) -> Fut,
    Fut: Future<Output = Out> + 'a + Send,
{
    type Fut = Fut;
}

impl App {
	pub fn setFunc(&mut self, path: &'static str, f: fn(&mut Context) -> BoxFuture<'_, HttpResult>) -> &mut App
	{
		self.routes.push(Route {
			regex: Regex::new(path).unwrap(),
			func: f,
		});
		return self;
	}

	pub fn new() -> Self {
		App {
			addr: ([0, 0, 0, 0], 8800),
			routes: vec![],
		}
	}

	pub fn listenAddress(&mut self, addr: ([u8; 4], u16)) -> &mut App {
		self.addr = addr;
		return self;
	}

	pub fn start(self) -> Result<()> {
		setup_logger();

		let _: Result<(), std::io::Error> = smol::block_on(async {
			let listener = Async::<TcpListener>::bind(self.addr).unwrap();
			println!("Listening on {}", listener.get_ref().local_addr()?);
			loop {
				let routes = self.routes.clone();
				let (stream, peer_addr) = listener.accept().await?;
				println!("Accepted client: {}", peer_addr);

				let stream = async_dup::Arc::new(stream);

				// Spawn a task that echoes messages from the client back to it.
				smol::spawn(async move {
					let routes = &routes.clone();
					if let Err(err) = async_h1::accept(stream, async move |req| {
						println!("Serving {}", req.url());

						let mut ctx = Context {
							pathIndex: 0,
							request: req,
							response: Response::new(StatusCode::NotFound),
							sessionData: Default::default(),
						};

						for route in routes.iter() {
							let url = ctx.request.url().as_str();
							if route.regex.is_match(url) {
								match (route.func)(&mut ctx).await {
									Ok(_) => {
										return Ok(ctx.response);
									}
									Err(err) => {
										let mut resp = Response::new(err.status());
										let msg = format!("{}", err);
										log::error!("{}", msg);
										resp.set_body(msg);
										return Ok(resp);
									}
								}
							}
						}

						return Ok(ctx.response);

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

// impl <F: for<'a> AsyncFn<'a, HttpResult>> Default for App<F> {
// 	fn default() -> Self {
// 		Self::new()
// 	}
// }
