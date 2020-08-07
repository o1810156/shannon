extern crate bimap;

use bimap::BiMap;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};

fn shannon_code(mut source: Vec<(f64, String)>, have_to_sort: bool) -> BiMap<String, String> {
    if source.len() <= 1 {
        panic!("please pass the source include at least two elements.");
    }

    if have_to_sort {
        // source.sort();
        // https://users.rust-lang.org/t/how-to-sort-a-vec-of-floats/2838
        source.sort_by(|a, b| a.partial_cmp(b).unwrap());
        source.reverse();
    }

    let mut book = BiMap::new();

    shannon_rec(&source, &mut book, String::new());

    book
}

fn shannon_rec(slce: &[(f64, String)], book: &mut BiMap<String, String>, code: String) {
    if slce.len() == 1 {
        book.insert(String::from(&slce[0].1), String::from(code));
        return;
    }

    let mut sum = 0f64;
    let sum_list = slce
        .iter()
        .map(|(f, _)| {
            sum += *f;
            sum
        })
        .collect::<Vec<_>>();

    let mut diff_min = sum;
    let mut min_index = 0;
    for i in 0..(slce.len() - 1) {
        let up = sum_list[i];
        let down = sum - sum_list[i];
        let diff = (up - down).abs();
        if diff_min > diff {
            diff_min = diff;
            min_index = i;
        }
    }

    let p = min_index + 1;
    shannon_rec(&slce[..p], book, format!("{}0", code));
    shannon_rec(&slce[p..], book, format!("{}1", code));
}

fn main() -> io::Result<()> {
    let mut args = env::args();
    args.next();
    let w_filename = args.next();
    if w_filename.is_none() {
        return Err(Error::new(ErrorKind::NotFound, "Please Pass Filename."));
    }

    let filename = w_filename.unwrap();
    let mut f = File::open(filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let list = contents
        .split('\n')
        .map(|line| {
            let mut t = line.split(',');
            let k = t.next().unwrap_or("");
            let v = t.next().unwrap_or("0");
            (v.parse::<f64>().unwrap_or(0f64), String::from(k))
        })
        .collect::<Vec<_>>();

    let entropy = list.iter().map(|(v, _)| v * (-v.log2())).sum::<f64>();
    let book = shannon_code(list.clone(), false);

    let avg = list
        .iter()
        .map(|(f, k)| (book.get_by_left(k).unwrap().len() as f64) * (*f))
        .sum::<f64>();

    let code_list = list
        .iter()
        .map(|(_, k)| format!("{} => {}", k, book.get_by_left(k).unwrap()))
        .collect::<Vec<_>>()
        .join("\n");

    println!("entropy: {}\n{}\navg_len: {}", entropy, code_list, avg);

    Ok(())
}

/*
    let l = source.len();

    // ナイーブな実装になってしまった...
    for s in &source[..l - 1] {
        book.insert(String::from(&s.1), format!("{}0", code));
        code.push_str("1");
    }

    book.insert(source.pop().unwrap().1, code);
*/
