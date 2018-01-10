fn hash(key: u64, num_servers: u64, lcng_a: u64) -> (u64) {
    assert!(num_servers > 0);

    let mut b :u64 = 0;
    let mut j :u64 = 0;
    let mut t_key = key;

    while j < num_servers {
        b = j;
//        println!("B: {}", b);

        t_key = t_key.wrapping_mul(lcng_a) + 1;
//        println!("KEY: {}", t_key);

        j = (((b + 1) as f64) * (((1u64 << 31) as f64) / (((t_key >> 33) + 1) as f64))) as u64;
//        println!("J: {}", j);
    }

    return b;
}

fn main() {
//    for i in 0..10 {
//        println!("{}", hash(i, 3));
//    }

    let total = 1000000;

//    let mut lcng_1 = 2862933555777941757;

    for lcng in [2862933555777941757u64, 3202034522624059733u64, 3935559000370003845u64].iter() {
        let mut same = 0;
        let mut diff = 0;

        for i in 0..total {
            if hash(i, 1000, *lcng) == hash(i, 999, *lcng) {
                same += 1;
            } else {
                diff += 1;
            }
        }

        println!("SAME: {} {}", same, same as f64 / total as f64);
        println!("DIFF: {} {}", diff, diff as f64 / total as f64);
    }


}
