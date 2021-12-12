use futures::{future, FutureExt}; 

async fn say() -> String{
	println!("hello");
	return "hello".to_string();
}

async fn greet() -> String{
	println!("world");
	return "world".to_string();
}

pub async fn speak() {
	// https://stackoverflow.com/questions/63463579/how-to-execute-multiple-async-functions-at-once-and-get-the-results
	
	let mut futures = Vec::new();
	futures.push(say().boxed());
	futures.push(greet().boxed());
	let results = future::join_all(futures).await;

	dbg!(&results);
}