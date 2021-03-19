mod application;
mod context;
mod request;
mod response;


#[cfg(test)]
mod tests {
	use crate::application::{App, AsyncFnPtr};
	use crate::haha;

	#[test]
	fn it_works() {
		let _ = smol::block_on(async {
			let v = vec![AsyncFnPtr::new(hehe)];
			App::setHandler(v, haha).await
		});
	}

	async fn hehe(a: i32, b: i32) -> bool {
		return a + b > 0i32;
	}
}



async fn haha(a: i32, b: i32)-> bool{
	println!("haha {}", a + b);
	return true;
}