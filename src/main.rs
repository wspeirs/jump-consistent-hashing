fn hash(key: u64, num_servers: u64) -> (u64) {
    assert!(num_servers > 0);

    let mut b :u64 = 0;
    let mut j :u64 = 0;
    let mut t_key = key;

    while j < num_servers {
        b = j;
//        println!("B: {}", b);

        t_key = t_key.wrapping_mul(2862933555777941757) + 1;
//        println!("KEY: {}", t_key);

        j = (((b + 1) as f64) * (((1u64 << 31) as f64) / (((t_key >> 33) + 1) as f64))) as u64;
//        println!("J: {}", j);
    }

    return b;
}

fn main() {
    for i in 0..10 {
        println!("{}", hash(i, 3));
    }

    for i in 0..10 {
        println!("{}", hash(i, 4));
    }

}
