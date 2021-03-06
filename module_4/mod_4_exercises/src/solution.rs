use std::f64::consts::PI;
use std::thread;
use std::sync::{Arc, RwLock, Mutex};

pub trait Geometry {
    fn get_area(&self) -> f64; // return area of shape
    fn get_name(&self) -> String; // return name of shape
}

pub struct Rectangle {
    pub length: f64,
    pub width: f64,
}

impl Geometry for Rectangle {}


pub struct Circle {
    pub radius: f64,
}

impl Geometry for Circle {}

struct Counter {
    count: i32
}

// define a function incr() to increment count in Counter by 1
fn incr() {}

fn counter() {
    // declare a counter

    // spawn a thread here to call incr() 50 times
    let handle = thread::spawn(move|| {
        for _i in [??] {
            println!("thread spawned count {}", [??]);
        }
    });

    // in the main thread, call incr() 50 times
    for _i in [??] {
        println!("thread main count {}", [??]);
    }
    handle.join().unwrap();
}


/*
 *  Modify the following function to use RwLocks. Run both blocks, one with standard mutexes
 *  and the other with RwLocks. What differences do you observe in their behavior/output? Does
 *  this match your understanding of how Read/Write locks work?
 */
fn read_write() {
    let lock = Arc::new(Mutex::new(0));
    let mut handles = Vec::with_capacity(10);

    for _i in 0..10 {
        let reader_lock = lock.clone();
        let reader = thread::spawn(move || {
            for _j in 0..20 {
                let r = reader_lock.lock().unwrap();
                println!("Read value as {}", *r);
            }
        });
        handles.push(reader)
    }

    for _j in 0..20 {
        let mut val = lock.lock().unwrap();
        *val += 1;
        println!("Incremented value by 1 to {}", *val);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
