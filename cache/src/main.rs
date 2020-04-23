fn main() {
    let mut store = cache::storage::new();
    store.set(&String::from("foo"), String::from("bar"));
    let res = store.get(&String::from("foo")).unwrap();
    println!("value is {}", res);
}
