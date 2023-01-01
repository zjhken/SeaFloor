use core::future::Future;
use futures::future::{BoxFuture, FutureExt};

async fn haha(ctx: &mut u8) -> u8 {
	dbg!(*ctx)
}

// 方式一：定义辅助 trait，仅适用于函数
trait AsyncFn<'a, Out>: Fn(&'a mut u8) -> Self::Fut {
	type Fut: Future<Output = Out> + 'a;
}

impl<'a, F, Out, Fut> AsyncFn<'a, Out> for F
where
	F: Fn(&'a mut u8) -> Fut,
	Fut: Future<Output = Out> + 'a,
{
	type Fut = Fut;
}

async fn ok1<F: for<'a> AsyncFn<'a, u8>>(f: F) {
	let mut a = 1;
	f(&mut a).await;
}

// 方式二：BoxFuture，适用于函数和闭包
async fn ok2(f: fn(&mut u8) -> BoxFuture<'_, u8>) {
	let mut a = 2;
	f(&mut a).await;
}

async fn ok3(f: &dyn Fn(&mut u8) -> BoxFuture<'_, u8>) {
	let mut a = 3;
	f(&mut a).await;
}

async fn ok4(f: Box<dyn Fn(&mut u8) -> BoxFuture<'_, u8>>) {
	let mut a = 4;
	f(&mut a).await;
}

#[tokio::main]
async fn main() {
	ok1(haha).await;
	ok2(|ctx| async move { haha(ctx).await }.boxed()).await;
	ok3(&|ctx| async move { haha(ctx).await }.boxed()).await;
	ok4(Box::new(|ctx| async move { haha(ctx).await }.boxed())).await;
}

// 一个案例：
// https://users.rust-lang.org/t/async-function-taking-a-reference-lifetimes-problem/83252
