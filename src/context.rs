#![allow(non_snake_case)]

use http_types::{Request, Response};

use std::collections::HashMap;
use std::fmt::Display;

pub struct Context {
	pub pathIndex: usize,
	pub request: Request,
	pub response: Response,
	pub sessionData: HashMap<&'static str, Box<dyn Display + Send + Sync>>,
}
