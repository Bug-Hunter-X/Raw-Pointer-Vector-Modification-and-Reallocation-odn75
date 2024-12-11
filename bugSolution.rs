fn main() {
    let mut v = vec![1, 2, 3];
    // Instead of using a raw pointer, use safe methods like indexing:
    v[0] = 10;
    println!("v: {:?}", v);
    
    // Alternative if index is calculated at runtime:
    let index = 0;
    if index < v.len(){
        v[index] = 10;
    }else{
        println!("Index out of bounds");
    }
    println!("v: {:?}", v);
} 