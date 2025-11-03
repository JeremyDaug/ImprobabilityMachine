use std::{collections::HashMap, fs::File, io::Write, path::Path};

use crate::bang::bang::Bang;

pub mod bang;

static BANG_CHANCES: &str = "./src/bang/bang_chances.txt";

/// # Bang Probabilities
/// 
/// Helper function to calculate 
pub fn bang_probs() {
    let mut results = HashMap::new();

    let mut bang = Bang::new();

    // iterate over each possibility (dumb option, but works.)
    for int in 0..(6_i32.pow(5)) {
        let a = int % 6 + 1;
        let b = int % (6*6) / 6 + 1;
        let c = int % (6*6*6) / (6*6) + 1;
        let d = int % (6*6*6*6) / (6*6*6) + 1;
        let e = int % (6*6*6*6*6) / (6*6*6*6) + 1;

        bang.result[0] = a as u8;
        bang.result[1] = b as u8;
        bang.result[2] = c as u8;
        bang.result[3] = d as u8;
        bang.result[4] = e as u8;

        let v = bang.scoring().0;
        //println!("{}", v);
        results.entry(v)
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }

    let mut output = String::new();
    let total_count: i32 = results.values().sum();
    output += format!("{}", total_count).as_str();
    println!("Total Count: {}\n", total_count);
    let mut keys = results.keys().collect::<Vec<&usize>>();
    keys.sort();
    
    // consolidate and print to file.
    for key in keys {
        let val = results.get(key).unwrap();
        println!("{} : {}", key, val);
        output += format!("{} : {}\n", key, val).as_str();
    }

    let path = Path::new(BANG_CHANCES);
    let mut file = File::create(path).unwrap();
    file.write(output.as_bytes()).unwrap();
}