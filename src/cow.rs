use std::borrow::Cow;

struct Logs<'a> {
    arr: Vec<Cow<'a, str>>,
}

impl<'a> Logs<'a> {
    pub fn add<T>(&mut self, s: T)
    where
        T: Into<Cow<'a, str>>,
    {
        self.arr.push(s.into());
    }
}

fn print<'a, T: Into<Cow<'a, str>>>(s: T) {
    println!("{}", s.into());
}

fn add_to_log<'a, T: Into<Cow<'a, str>>>(logs: &mut Logs<'a>, s: T) {
    logs.arr.push(s.into());
}

pub fn cow_array() {
    // Create an array of `Cow<str>` containing both `&'static str` and `String`.
    let arr: Vec<Cow<str>> = vec![
        Cow::Borrowed("Hello, world!"),           // `&'static str`
        Cow::Owned(String::from("Hello, Rust!")), // `String`
    ];

    // Iterate over the array and print each item.
    for item in &arr {
        println!("{}", item);
    }

    let s1 = "Hello &str!!";
    let s2 = "Hello String!!".to_string();
    print(s1);
    print(s2);

    let s3 = "cat";
    let s4 = "dog".to_string();

    let mut l = Logs { arr: Vec::new() };
    add_to_log(&mut l, s3);
    add_to_log(&mut l, s4);

    let s5 = "bear";
    let s6 = "tiger".to_string();
    l.add(s5);
    l.add(s6);

    for i in &l.arr {
        println!("{}", i);
    }
}
