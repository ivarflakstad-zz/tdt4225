extern crate time;
extern crate num;

use std::io::Write;
use std::fs::File;
use std::fs;
use std::path::Path;

const BLOCKSIZE : usize = 8_192;
static NBLOCKS : usize = 131_092; //262_144;
static NTESTS : usize = 6;

fn mb_per_sec(gb: usize, time: &u64) -> f32 {
    // Takes in GB and time since epoch in ns, returns MB/s
    let mbps = (gb * 1000) as f32 / (nsu64_to_msf32(time)/ 1000.0);
    mbps
}
fn ns_to_ms(ns: f32) -> f32 {
    // Scales nanoseconds to milliseconds
    let ms = ns / (num::pow(1000.0, 2));
    ms
}
fn u64_to_f32(u: &u64) -> f32 {
    // Simple type converter
    let f = num::traits::ToPrimitive::to_f32(u).expect("Can not convert to f32");
    f
}
fn nsu64_to_msf32(ns: &u64) -> f32 {
    // Combines the two functions above
    let ms = ns_to_ms(u64_to_f32(ns));
    ms
}

fn main() {
    println!("*-----------------------------------------------*");
    println!("|TDT4225 Exercise 2: File systems               |");
    println!("|I/O programming, write capacity                |");
    println!("|Testing writing speed of blocksizes 1 to 32 GB |");
    println!("*-----------------------------------------------*");

    let gb_sizes = [1, 2, 4, 8, 16, 32];
    let mut times = [0, 0, 0, 0, 0, 0];
    let buff = [0u8; BLOCKSIZE];
    {
        let start_time = time::precise_time_ns();
        for i in 0..NTESTS {
            let mut f = File::create("./foo.txt").ok().expect("Could not create file");
            let size = gb_sizes[i];
            let t1 = time::precise_time_ns();
            for _ in 0..(size*NBLOCKS){
                f.write(&buff).ok().expect("Could not write to file");
            }
            let t2 = time::precise_time_ns();

            times[i] = t2-t1;

            fs::remove_file(&Path::new("./foo.txt")).unwrap_or_else(|why| {
                panic!("Error! {}", why)
            });
        }
        let tot_time = time::precise_time_ns() - start_time;
        let fmtd_tot_time = format!("{:.*}", 3, nsu64_to_msf32(&tot_time));
        println!("\t Total time: {} ms", fmtd_tot_time);

        //println!("Total time: {} ms", format!("{:.*}", 3, nsu64_to_msf32(tot_time)));
    }

    println!("_______________________________________________");
    println!("| {0: <5} | {1: <14} | {2: <15} |", "Rust I/O", "Throughput", "Time");
    /*
    println!("{0: <10} | {1: <10} | {2: <10}", 0, 0, 0);
    println!("{0: <10} | {1: <10} | {2: <10}}", 77, 0, 3);
    println!("{0: <10} | {1: <10} | {2: <10}", 112, 0, 6);
    println!("{0: <10} | {1: <10} | {2: <10}", 460, 0, 10);*/
    let mut gb;
    for (i, time) in times.iter().enumerate() {
        gb = gb_sizes[i];
        let fmtd_time = format!("{:.*}", 3, nsu64_to_msf32(time));
        let fmtd_mpbs = format!("{:.*}", 3, mb_per_sec(gb, time));
        println!("| {0: <5} GB | {1: <9} MB/s | {2: <12} ms |", gb, fmtd_mpbs, fmtd_time);
    }
    println!("_______________________________________________");
}
