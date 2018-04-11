//#[macro_use] extern crate lazy_static;
extern crate chrono;
extern crate durationfmt;
extern crate failure;
extern crate quickersort;
extern crate rayon;
extern crate regex;
extern crate walkdir;

use std::path::Path;
use std::fs;
use std::result::Result;
use failure::Error;
use regex::RegexSet;
use walkdir::WalkDir;
// use rayon::prelude::*;
use chrono::prelude::*;
pub mod clock;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}

    #[test]
    fn clock_ticking() {
        use std::time;
        // use clock::Clock;
        use durationfmt;

        let d: time::Duration = time::Duration::new(90, 0);
        assert_eq!("1m30s", durationfmt::to_string(d));
    }
}

pub fn file_list(
    root_list: &[&str],
    pattern_list: &[&str],
    verbose: bool,
) -> Result<Vec<String>, Error> {
    let re = RegexSet::new(pattern_list).unwrap();

    if verbose {
        println!("Processing...");
    }

    let mut fpl = vec![];

    for root in root_list.iter() {
        let walker = WalkDir::new(root).into_iter();
        let mut fpl_tmp = vec![];

        for entry in walker
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| re.is_match(e.path().to_str().unwrap()))
        {
            let e = entry.path().to_str().unwrap();
            fpl_tmp.push(e.to_string());
        }
        let _fpl_len = fpl_tmp.len() as i32;
        if verbose {
            println!("    > {:>5} files in {}", _fpl_len, root);
        }
        fpl.extend(fpl_tmp);
    }
    if verbose {
        println!("\t>> Total files: {}.\n", fpl.len(),);
    }
    quickersort::sort(&mut fpl[..]);
    if verbose {
        for (i, entry) in fpl.iter().enumerate() {
            println!("{:>4}\t {}", i, entry);
        }
        println!("\n>> Total files: {}.", fpl.len(),);
    }
    Ok(fpl)
}

pub fn create_parent_directory(file_name: &str) -> Result<(), Error> {
    //Recursively create all of the parent components of a file if they are missing.
    let dir_name: &str = Path::new(file_name).parent().unwrap().to_str().unwrap();
    if !Path::new(dir_name).exists() {
        // fs::create_dir_all(dir_name).unwrap();
        fs::create_dir_all(dir_name)?;
    }
    Ok(())
}

pub fn quick_sort<T: PartialOrd + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let mid = partition(v);
    let (lo, hi) = v.split_at_mut(mid);
    rayon::join(|| quick_sort(lo), || quick_sort(hi));
}

fn partition<T: PartialOrd + Send>(v: &mut [T]) -> usize {
    let pivot = v.len() - 1;
    let mut i = 0;
    for j in 0..pivot {
        if v[j] <= v[pivot] {
            v.swap(i, j);
            i += 1;
        }
    }
    v.swap(i, pivot);
    i
}

pub fn duration2string(dt: chrono::Duration) -> String {
    (Utc.ymd(1970, 1, 1).and_hms(0, 0, 0) + dt)
        .format("%Hh:%Mm:%Ss")
        .to_string()
}

pub fn chrono_date2num(dt: DateTime<Utc>) -> f64 {
    //Convert a DateTime from Chrono to a float according to the default values of python pylab :
    //"units" = "days since 0001-01-01T00:00Z"
    //"calendar" = "proleptic_gregorian"
    // -1     = retrieve this number of day to match pylab datetime conversion
    // 24.0   = hours in one day
    // 1440   = minutes in one day
    // 86,400 = seconds in one day
    //const DAY_OFFSET: f64 = 1.0;
    //lazy_static! {
    //static ref DAY_OFFSET: f64 = 1.0;
    //}
    let f: f64 = (dt.num_days_from_ce() as f64 - 1.0) + (dt.hour() as f64) / 24.0
        + (dt.minute() as f64) / 1440.0 + (dt.second() as f64) / 86400.0;
    f
}

// use chrono;
// use durationfmt;

// pub struct Clock {
//     stored_time: chrono::DateTime<chrono::Utc>,
// }
// impl Clock {
//     pub fn new() -> Clock {
//         Clock { stored_time: chrono::Utc::now() }
//     }
//     pub fn tick(&mut self) -> &mut Clock {
//         self.stored_time = chrono::Utc::now();
//         self
//     }
//     pub fn tock(self) -> String {
//         let time_spent: chrono::Duration = chrono::Utc::now().signed_duration_since(self.stored_time);
//         durationfmt::to_string(time_spent.to_std().unwrap())
//     }
// }
