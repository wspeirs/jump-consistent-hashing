//use std::collections::BTreeSet;
use std::error::Error;

fn assign_servers(key: u64, num_servers: u64, copies: usize) -> Result<Vec<u64>, Box<Error>> {
    // constants from: http://www.ams.org/journals/mcom/1999-68-225/S0025-5718-99-00996-5/S0025-5718-99-00996-5.pdf
    let constants = [
        2862933555777941757u64,
        3202034522624059733u64,
        3935559000370003845u64,
        1181783497276652981u64,
        4292484099903637661u64,
        7664345821815920749u64,
        1865811235122147685u64,
        2685821657736338717u64,
        1803442709493370165u64
    ];

    if num_servers == 0 || num_servers < copies as u64 {
        return Err("num_servers is invalid: either 0 or < copies".into());
    }

    let mut ret = Vec::with_capacity(copies);

    // go through and hash the key with each constant
    for c in constants[0..copies].iter() {
        let mut s = hash(key, num_servers, *c);

        // while we're already matching a server pick, move on to the next one
        while ret.iter().any(|&x| x == s) {
            s = (s + 1) % copies as u64;
        }

        ret.push(s);
    }

    return Ok(ret);
}

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

    let total = 10; // 00000;

    for i in 0..total {
        let servers = assign_servers(i, 6, 5);
        let s_list = servers.unwrap().iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");

        println!("{}", s_list);
    }

/*
    // constants from: http://www.ams.org/journals/mcom/1999-68-225/S0025-5718-99-00996-5/S0025-5718-99-00996-5.pdf
    let constants = [
        2862933555777941757u64,
        3202034522624059733u64,
        3935559000370003845u64,
        1181783497276652981u64,
        4292484099903637661u64,
        7664345821815920749u64,
        1865811235122147685u64,
        2685821657736338717u64,
        1803442709493370165u64
    ];

    let mut no_overlap = 0;

    for i in 0..total {
        let mut servers = BTreeSet::new();

        for lcng in constants.iter() {
            servers.insert(hash(i, 1000, *lcng));
        }

        if servers.len() == constants.len() {
            no_overlap += 1;
        }
    }

    println!("{} of {} had no overlap; {}", no_overlap, total, (no_overlap as f64 / total as f64) * 100.0);
*/
/*
    for lcng in constants.iter() {
        let mut same = 0;
        let mut diff = 0;

        for i in 0..total {
            if hash(i, 1000, *lcng) == hash(i, 999, *lcng) {
                same += 1;
            } else {
                diff += 1;
            }
        }

        println!("{}", lcng);
        println!("SAME: {} {}", same, same as f64 / total as f64);
        println!("DIFF: {} {}", diff, diff as f64 / total as f64);
        println!();
    }
*/

}
