use serde_json::Value;

pub fn run() {
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