#[test]
fn t0() {
    fn run() {
        use serde_json::Value;

        let test_json = r#"
        {
            "a":"a_val",
            "b":{
                "b_sub" : "b_sub_val"
            },
            "d":[
                {
                    "d0" : "d0_val"
                },
                {
                    "d1" : "d1_val"
                }
            ]
        }
        "#;

        let val: Value = serde_json::from_str(test_json).unwrap();

        println!("{}", val["a"]);
        println!("{}", val["b"]["b_sub"]);
        println!("{}", val["b"]["b_sub"].to_string());
        println!("{}", val["b"]["b_sub"].as_str().unwrap());
        println!("{}", val["d"][0]["d0"].as_str().unwrap());
        println!("{}", val["no_exist"]["what_will_happen"]);
    }

    fn kick(title: &str, f: fn()) {
        println!("====================");
        println! {" {}", title};
        println!("====================");

        f();

        println!("\n");
    }

    kick("Parse JSON", run);
}

#[test]
fn t1() {
    let c = 3;

    let s = match c {
        0..=1 => "x",
        2..=10 => "y",
        _ => "z",
    };
    println!("Hello, world! {}", s);
}

pub mod threading;
#[test]
fn t2() {
    threading::test_threading();
}

#[test]
fn t3() {
    use std::collections::HashMap;

    let mut p = HashMap::new();
    let k1: [u8; 2] = [1, 2];
    let k2: [u8; 2] = [2, 2];

    p.insert(k1, "v1");
    p.insert(k2, "v2");

    dbg!(&p.get(&[2, 2]));

    let mut p1 = HashMap::new();
    let kk1: Vec<u8> = vec![0, 1, 2];
    let kk2: Vec<u8> = vec![0, 1];
    p1.insert(kk1, "v1");
    p1.insert(kk2, "v2");

    dbg!(&p1.get(&vec![0, 1, 2]));
}

#[test]
fn t4() {
    use hdrhistogram::Histogram;

    // Must be in the range [0, 5]. If you're not sure, use 3.
    let mut hist = hdrhistogram::Histogram::<u64>::new(1).unwrap();

    for n in 1..101 {
        hist.record(54321 + n).unwrap();
    }
    hist.record(54321).unwrap();
    hist.record(54300).unwrap();
    hist.record(1).unwrap();
    hist.record(10).unwrap();
    hist.record(10).unwrap();
    hist.record(10).unwrap();
    hist.record(10).unwrap();
    hist.record(5).unwrap();
    hist.record(4).unwrap();
    hist += 2;

    let hist_const = Histogram::<u64>::new(1)
        .and_then(|mut hist| {
            hist += 2;
            Ok(hist)
        })
        .unwrap();

    hist -= hist_const;

    let iter: Vec<(u64, u64)> = hist
        .iter_recorded()
        .map(|v| (v.value_iterated_to(), v.count_at_value()))
        .collect();
    dbg!(&iter);
}

#[test]
fn t5() {
    use std::sync::mpsc::channel;
    use std::{thread, time};

    let (sender, receiver) = channel();

    // Spawn off an expensive computation
    thread::spawn(move || {
        sender.send("xxx".to_string()).unwrap();
        drop(sender)
    });

    // Do some useful work for awhile
    thread::sleep(time::Duration::from_secs_f32(1.0));

    // Let's see what that answer was
    match receiver.recv() {
        Ok(result) => println!("{}", result),
        Err(e) => println!("{}", e),
    }

    match receiver.recv() {
        Ok(result) => println!("{}", result),
        Err(e) => println!("{}", e),
    }
}

#[test]
fn t6() {
    use flate2::bufread::GzDecoder;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::Read;
    use std::path::Path;

    let p = Path::new("/Users/gu/git/cache-trace/captures/sample_data_full/xgu_ewr18120_gcs_data/cache-ewr18120_2022-06-26_232406847133.gz");
    let f = File::open(p).unwrap();
    let f = BufReader::with_capacity(1024 * 1024, f);
    let mut f = GzDecoder::new(f);
    let mut buffer = Vec::<u8>::new();

    let size = f.read_to_end(&mut buffer).unwrap();
    dbg!(size, buffer.len());
}

#[test]
fn t7() {
    use serde::Deserialize;

    //https://learnxinyminutes.com/docs/toml/
    #[derive(Debug, Deserialize)]
    struct Group {
        name: String,
        size: usize,
    }

    #[derive(Debug, Deserialize)]
    struct Config {
        groups: Vec<Group>,
        a_setting: usize,
    }

    let sample = r#"
        a_setting = 1234
        [[groups]]
        name = "medium"
        size = 10

        [[groups]]
        name = "large"
        size = 100"#;

    let cfg: Config = toml::de::from_str(sample).unwrap();
    println!(
        "{} {} {}",
        cfg.groups[0].name, cfg.groups[0].size, cfg.a_setting
    )
}

#[test]
fn t8() {
    use ipnet::Ipv4Net;

    // serialize Vec IpNet to binary
    // https://docs.rs/ipnet/latest/ipnet/
    // For compact binary formats (e.g. Bincode) the Ipv4Net and Ipv6Net types will serialize to a string of 5 and 17 bytes that consist of the network address octects followed by the prefix length. The IpNet type will serialize to an Enum with the V4 or V6 variant index prepending the above string of 5 or 17 bytes.
    let block_ip: Ipv4Net = "192.168.0.0/24".parse::<Ipv4Net>().unwrap();
    let mut ip_vec = Vec::new();
    for _ in 0..10 {
        ip_vec.push(block_ip.clone());
    }
    let binary_ipnet_vec: Vec<u8> = bincode::serialize(&ip_vec).unwrap();
    dbg!(&binary_ipnet_vec);

    // TODO: save binary_ipnet_vec to object store
    // read binary_ipnet_vec from object store

    let restored_ipnet_vec: Vec<Ipv4Net> = bincode::deserialize(&binary_ipnet_vec).unwrap();
    dbg!(&restored_ipnet_vec);
}

#[test]
fn t9() {
    use arc_cache::ArcCache;
    use std::hash::Hash;

    trait TestPrint {
        fn print(&self);
    }

    impl<K, V> TestPrint for ArcCache<K, V>
    where
        K: Eq + Hash,
    {
        fn print(&self) {
            println!("{:?}", &self.len());
        }
    }

    // https://github.com/jedisct1/rust-arc-cache/blob/master/src/lib.rs
    let mut cache = ArcCache::new(10).unwrap();
    cache.insert("abc", "efg");
    cache.print();
}

pub mod join;
#[tokio::test]
async fn t10() {
    use join;
    join::spawn::run().await;
    join::concurrent::speak().await;
    assert!(true);
}

pub mod container;
#[test]
fn t11() {
    use container::*;
    test_box();
    test_rc();
    test_arc();
    test_rwlock();
    test_cell();
    test_rc_cell();
    test_global_mutable1();
    test_global_mutable2();
    test_atomic_usize();
    test_rwlock();
}
