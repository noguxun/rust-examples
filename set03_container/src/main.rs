#[macro_use]
extern crate lazy_static;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

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
#[allow(dead_code)]
fn test_box() {
    let boxed_point = Box::new(Point { x: 1, y: 2 });
    println!("{} {}", boxed_point.x, boxed_point.y);
}

// Rc<T> Rc<T> (short for Reference Counting)
// https://abronan.com/rust-trait-objects-box-and-rc/
// used when we want multiple methods using a read only reference thus providing with shared ownership over some content.
// c, unlike Box, doesR not copy the whole context and data when calling clone, it only copies and hands-off a reference to the object on the heap, the "fat pointer" with the virtual table pointing to the right Trait implementation
#[allow(dead_code)]
fn test_rc() {
    let rc_a = Rc::new("example".to_string());
    let rc_b = rc_a.clone();
    let rc_c = Rc::clone(&rc_a);

    println!("{} {} {}", rc_a, rc_b, rc_c);
}

// Arc<T> thread safe version of Rc<T> (the counter of Rc<T> is not thread safe?)
#[allow(dead_code)]
fn test_arc() {

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
#[allow(dead_code)]
fn test_cell() {
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
    println!("{}", my_struct.special_field.get());

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
#[allow(dead_code)]
fn test_rc_cell() {
    #[derive(Debug)]
    enum List {
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

lazy_static! {
    static ref HOSTNAME: Mutex<String> = Mutex::new(String::new());
}

// https://users.rust-lang.org/t/how-can-i-use-mutable-lazy-static/3751/4
#[allow(dead_code)]
fn test_global_mutable1() {
    HOSTNAME.lock().unwrap().push_str("something 1");
}

#[allow(dead_code)]
fn test_global_mutable2() {
    HOSTNAME.lock().unwrap().push_str("something 2");
}

#[allow(dead_code)]
// https://doc.rust-lang.org/std/sync/atomic/
fn test_atomic_usize() {
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

fn main() {
    //test_box();
    //test_rc();
    //test_arc();
    //test_cell();
    //test_rc_cell();

    //test_global_mutable1();
    //test_global_mutable2();
    //print!("{:?}", HOSTNAME.lock().unwrap())

    test_atomic_usize();
}
