extern crate redis;
use redis::Commands;

fn main() {
    // Connect to Redis
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();

    // Set a key-value pair
    let _: () = con.set("my_key", 42).unwrap();

    // Get the value of a key
    let my_key: i32 = con.get("my_key").unwrap();
    println!("Value of my_key: {}", my_key);

    // Increment a value
    let _: () = con.incr("my_key", 10).unwrap();

    // Get the updated value
    let my_key_updated: i32 = con.get("my_key").unwrap();
    println!("Updated value of my_key: {}", my_key_updated);
}

