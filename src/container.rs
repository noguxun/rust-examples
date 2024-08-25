use lazy_static::lazy_static;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

// https://github.com/usagi/rust-memory-container-cs
// https://docs.google.com/presentation/d/1q-c7UAyrUlM-eZyTo1pd8SZ0qwA_wYxmPZVOQkoDmH4/edit#slide=id.p
// http://cosmic.mearie.org/2014/01/periodic-table-of-rust-types/

// Box<T> allocats T on heap
struct Point {
    x: i32,
    y: i32,
}

// Box let you alloc on heap
// https://users.rust-lang.org/t/when-should-i-use-box-t/4411/8
pub fn test_box() {
    let boxed_point = Box::new(Point { x: 1, y: 2 });
    println!("{} {}", boxed_point.x, boxed_point.y);
}

// Rc<T> Rc<T> (short for Reference Counting)
// https://abronan.com/rust-trait-objects-box-and-rc/
// used when we want multiple methods using a read only reference thus providing with shared ownership over some content.
// c, unlike Box, doesR not copy the whole context and data when calling clone, it only copies and hands-off a reference to the object on the heap, the "fat pointer" with the virtual table pointing to the right Trait implementation
pub fn test_rc() {
    let rc_a = Rc::new("example".to_string());
    let rc_b = rc_a.clone();
    let rc_c = Rc::clone(&rc_a);

    println!("{} {} {}", rc_a, rc_b, rc_c);
}

// Arc<T> thread safe version of Rc<T> (the counter of Rc<T> is not thread safe?)
pub fn test_arc() {
    let apple = Arc::new("the same apple".to_string());
    for _ in 0..10 {
        // Here there is no value specification as it is a pointer to a reference
        // in the memory heap.
        let apple = Arc::clone(&apple);

        thread::spawn(move || {
            // As Arc was used, threads can be spawned using the value allocated
            // in the Arc variable pointer's location.
            println!("{:?}", apple);
        });
    }
}

// https://stackoverflow.com/questions/30831037/situations-where-cell-or-refcell-is-the-best-choice
// https://abronan.com/rust-trait-objects-box-and-rc/
// Rc<RefCell<T>>
// Arc<Mutex<RefCell<T>>>

// https://doc.rust-lang.org/std/cell/struct.Cell.html
// https://www.reddit.com/r/rust/comments/755a5x/i_have_finally_understood_what_cell_and_refcell/
pub fn test_cell() {
    // Cell
    struct SomeStruct {
        regular_field: u8,
        special_field: Cell<u8>,
    }

    let my_struct = SomeStruct {
        regular_field: 0,
        special_field: Cell::new(1),
    };

    let new_value = 100;

    // ERROR: `my_struct` is immutable
    // my_struct.regular_field = new_value;

    // WORKS: although `my_struct` is immutable, `special_field` is a `Cell`,
    // which can always be mutated
    my_struct.special_field.set(new_value);
    my_struct.special_field.set(20);
    assert_eq!(my_struct.special_field.get(), 20);
    println!("{}", my_struct.special_field.get());
    println!("{}", my_struct.regular_field);

    // Test of RefCell
    let x = 42;
    // N.B. c is immutable.
    let rc = RefCell::new(x);

    {
        let borrowed_i_0 = rc.borrow();
        let borrowed_i_1 = rc.borrow();
        println!("{} {}", borrowed_i_0, borrowed_i_1);
    }

    // can do borrow mut with end of life of borrows
    *rc.borrow_mut() = 30;
    println!("{}", rc.borrow());
}

// http://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch15-05-interior-mutability.html
pub fn test_rc_cell() {
    #[derive(Debug)]
    enum List {
        #[allow(dead_code)]
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

// https://users.rust-lang.org/t/how-can-i-use-mutable-lazy-static/3751/4
lazy_static! {
    static ref HOSTNAME: Mutex<String> = Mutex::new(String::new());
}

pub fn test_global_mutable1() {
    let mut host_string = HOSTNAME.lock().unwrap();
    host_string.push_str("something 1.1 ");

    // Test drop for mutex, for releasing the lock
    drop(host_string);

    HOSTNAME.lock().unwrap().push_str("something 1.2 ");

    // will auto release after the scope
}

pub fn test_global_mutable2() {
    HOSTNAME.lock().unwrap().push_str("something 2 ");
}

// https://doc.rust-lang.org/std/sync/atomic/
pub fn test_atomic_usize() {
    let spinlock = Arc::new(AtomicUsize::new(1));

    let spinlock_clone = Arc::clone(&spinlock);
    let thread = thread::spawn(move || {
        spinlock_clone.store(0, Ordering::SeqCst);
    });

    // Wait for the other thread to release the lock
    while spinlock.load(Ordering::SeqCst) != 0 {}

    if let Err(panic) = thread.join() {
        println!("Thread had an error: {:?}", panic);
    }
}

// https://riptutorial.com/rust/example/24527/read-write-locks
pub fn test_rwlock() {
    // Create an u32 with an inital value of 0
    let initial_value = 0u32;

    // Move the initial value into the read-write lock which is wrapped into an atomic reference
    // counter in order to allow safe sharing.
    let rw_lock = Arc::new(RwLock::new(initial_value));

    // Create a clone for each thread
    let producer_lock = rw_lock.clone();
    let consumer_id_lock = rw_lock.clone();
    let consumer_square_lock = rw_lock;

    let producer_thread = thread::spawn(move || {
        for _ in 0..10 {
            // write() blocks this thread until write-exclusive access can be acquired and retuns an
            // RAII guard upon completion
            if let Ok(mut write_guard) = producer_lock.write() {
                // the returned write_guard implements `Deref` giving us easy access to the target value
                *write_guard += 1;

                println!("Updated value: {}", *write_guard);
            }

            // ^
            // |   when the RAII guard goes out of the scope, write access will be dropped, allowing
            // +~  other threads access the lock

            sleep(Duration::from_millis(100));
        }
    });

    // A reader thread that prints the current value to the screen
    let consumer_id_thread = thread::spawn(move || {
        for _ in 0..10 {
            // read() will only block when `producer_thread` is holding a write lock
            if let Ok(read_guard) = consumer_id_lock.read() {
                // the returned read_guard also implements `Deref`
                println!("Read value: {}", *read_guard);
            }

            sleep(Duration::from_millis(50));
        }
    });

    // A second reader thread is printing the squared value to the screen. Note that readers don't
    // block each other so `consumer_square_thread` can run simultaneously with `consumer_id_lock`.
    let consumer_square_thread = thread::spawn(move || {
        for _ in 0..10 {
            if let Ok(lock) = consumer_square_lock.read() {
                let value = *lock;
                println!("Read value squared: {}", value * value);
            }

            sleep(Duration::from_millis(70));
        }
    });

    let _ = producer_thread.join();
    let _ = consumer_id_thread.join();
    let _ = consumer_square_thread.join();
}
