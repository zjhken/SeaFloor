use std::{fmt::Display, vec};

use anyhow::Result;

use seafloor::{application::App, context::Context};

// todo: static file embedded
// todo: scheduler
// todo: file upload in stream
// todo: json
// todo: sqlite
// todo: msgpack
// todo: postgres

fn main() -> Result<()> {
	App::new()
		.setFunc("/test", hehe)
		.setFunc("/test.*", do_it)
		.listenAddress(([0, 0, 0, 0], 8800))
		.start()
}

async fn hehe(mut ctx: Context) -> Result<Context, http_types::Error> {
	println!("Enter hehe");
	ctx.response.set_body("This is hehe function");
	ctx.sessionData.insert(
		"user",
		Box::new(User {
			name: "Tom".to_string(),
			age: 3,
		}),
	);
	ctx.sessionData.insert(
		"list",
		Box::new(TheVec(vec!["haha".to_owned(), "hehe".to_owned()])),
	);
	let ctx = ctx.next().await;
	println!("hehe done");
	return ctx;
}

async fn do_it(mut ctx: Context) -> Result<Context, http_types::Error> {
	println!("Enter doIt");
	ctx.response.insert_header("Content-Type", "text/plain");
	let s = ctx.sessionData.get("user").unwrap();
	println!(">>>>>>>>>>{s}");
	// s.push_str("This is doIt function");
	ctx.response.set_body(s.to_string());
	ctx.sessionData.insert(
		"a",
		Box::new(User {
			name: "haha".to_owned(),
			age: 4u8,
		}),
	);
	let listStr = ctx.sessionData.get("list").unwrap().to_string();
	println!("{listStr}");
	let ctx = ctx.next().await;
	println!("DoIt done.");
	return ctx;
}

struct TheVec(Vec<String>);
impl Display for TheVec {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut comma_separated = String::new();

		for s in &self.0[0..self.0.len() - 1] {
			comma_separated.push_str(&s);
			comma_separated.push_str(", ");
		}

		comma_separated.push_str(&self.0[self.0.len() - 1].to_string());
		write!(f, "{}", comma_separated)
	}
}

struct User {
	name: String,
	age: u8,
}

impl std::fmt::Display for User {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.name)
	}
}
