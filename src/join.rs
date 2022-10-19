pub mod concurrent {
    use futures::{future, FutureExt};

    async fn say() -> String {
        println!("hello");
        "hello".to_string()
    }

    async fn greet() -> String {
        println!("world");
        "world".to_string()
    }

    pub async fn speak() {
        // https://stackoverflow.com/questions/63463579/how-to-execute-multiple-async-functions-at-once-and-get-the-results

        let futures = vec![say().boxed(), greet().boxed()];
        let results = future::join_all(futures).await;

        dbg!(&results);
    }
}

pub mod spawn {
    #[derive(Debug)]
    struct Data {
        resp: String,
    }

    async fn do_stuff1() -> Data {
        println!("do_stuff1 start");
        let client = reqwest::Client::new();

        let res = client
            .post("http://httpbin.org/post")
            .body("stuff1 body")
            .header("abc", "def")
            .send()
            .await
            .unwrap();

        let resp_str = res.text().await.unwrap();

        println!("do_stuff1 end");
        Data { resp: resp_str }
    }

    async fn do_stuff2() -> Data {
        println!("do_stuff2 start");
        let client = reqwest::Client::new();

        let res = client
            .post("http://httpbin.org/post")
            .body("stuff2 body")
            .header("123", "456")
            .send()
            .await
            .unwrap();

        let resp_str = res.text().await.unwrap();
        println!("do_stuff2 end");

        Data { resp: resp_str }
    }

    async fn do_stuff3(header: &str, value: String) -> Data {
        println!("dull_job start {}", value);
        let client = reqwest::Client::new();

        let res = client
            .post("http://httpbin.org/post")
            .body("the exact body that is sent")
            .header(header, value.clone())
            .send()
            .await
            .unwrap();

        let resp_str = res.text().await.unwrap();
        println!("dull_job end {}", value);

        Data { resp: resp_str }
    }

    pub async fn run() {
        // https://docs.rs/tokio/0.2.18/tokio/macro.join.html

        println!("Test tokio::join");
        let (first, second) = tokio::join!(do_stuff1(), do_stuff2());

        println!("first {}", first.resp);
        println!("second {}", second.resp);

        println!("Test tokio::spawn, await");
        let mut handle_vec = Vec::new();

        for i in 0..10 {
            let value = format!("{}", i);
            let handle = tokio::task::spawn(do_stuff3("xgu-debug", value));
            handle_vec.push(handle);
        }
        println!("all jobs are spanwed");

        while let Some(handle) = handle_vec.pop() {
            let data = handle.await.unwrap();
            println!("got {}", data.resp);
        }
    }
}
