use std::sync::mpsc;
use std::{thread, time};
use anyhow::Result;
use rayon::prelude::*;

// -------------------------------
fn process_job(msg: &str) {
    println!("some job with msg {}", msg);
}

fn basic_thread() {
    let handle1 = thread::spawn(|| {
        thread::sleep(time::Duration::from_millis(1));
        println!("hello from a child thread");
    });

    let msg = "water".to_string();

    let handle2 = thread::spawn(move || process_job(&msg));
    println!("Hello, world!");
    handle1.join().unwrap();
    handle2.join().unwrap();
}

// -------------------------------
fn sending_thread()->(mpsc::Receiver<String>, thread::JoinHandle<Result<()>>) {
    let (sender, receiver) = mpsc::channel();
    
    let handle = thread::spawn(move || {
        for i in 0..10 {
            let text = format!("msg: {}", i);
            if sender.send(text).is_err() {
                break;
            }
            thread::sleep(time::Duration::from_millis(1));
            println!("sent {}", i);
        }
        Ok(())
    });

    (receiver, handle)
}

fn channel() {
    let (receiver, handle) = sending_thread();

    for text in receiver {
        println!("{}", &text);
    }

    handle.join().unwrap().unwrap();
}

fn parall_iteration() {
    let v: Vec<_> = (0..10).collect();
    let w: Vec<_> = v.par_iter().map(|i| {println!("{}", i); i*i}).collect();
    println!("{:?}", w);
}

fn multiple_sender() {
    use std::sync::mpsc::{channel};

    let (r_sender, r_receiver) = mpsc::sync_channel(1);
    thread::spawn(move || {
        let n_jobs = 10;
        //let pool = ThreadPool::new(n_workers);

        //let mut time_vec = Vec::new();
        // let now = time::Instant::now();
        let mut handles = Vec::new();
        for i in 1..(n_jobs + 1) {
            let (l_sender, l_receiver) = channel();
            r_sender.send(l_receiver).unwrap();

            let handle = thread::spawn(move || {
                for j in 1..10 {
                    l_sender.send(format!("{}.{}", i, j)).unwrap();
                }
            });

            handles.push(handle);

            /*
            while pool.queued_count() > 1 {
                thread::sleep(time::Duration::from_secs_f32(1.0));
            } */
        }

        drop(r_sender);

        for _handle in handles {
            //_handle.join().unwrap();
        }

        //pool.join();
    });

    for l_r in r_receiver {
        //while let Ok(s) = l_r.recv() {
        for s in l_r {
            println!("{}", s);
        }
        //thread::sleep(time::Duration::from_secs_f32(1.0));
    }

    //for t in time_vec {
    //println!("{}", t.as_nanos())
    //}
}


pub fn test_threading() {
    basic_thread();
    channel();
    parall_iteration();
    multiple_sender();
}
