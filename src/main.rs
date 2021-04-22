use texide::scopedmap::ScopedMap;

fn main() {
    let mut map = ScopedMap::new();
    map.insert(1, 3);
    println!("{:?}", map.get(&1));
    println!("{:?}", map.get(&2));
    map.begin_scope();
    map.insert(1, 2);
    map.insert(2, 5);
    println!("{:?}", map.get(&1));
    println!("{:?}", map.get(&2));
    assert!(map.end_scope());
    println!("{:?}", map.get(&1));
    println!("{:?}", map.get(&2));
    println!("Hello, world!");
}
