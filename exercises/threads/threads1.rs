// threads1.rs
//
// This program spawns multiple threads that each run for at least 250ms, and
// each thread returns how much time they took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.


use std::thread;
use std::time::{Duration, Instant};

/*
thread::spawn 返回一个JoinHandle<T>，帮助控制线程并收集结果
    .join()等待线程完成，并返回结果Result<T,_>

*/
fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        handles.push(
            thread::spawn(move || {
                let start = Instant::now();
                thread::sleep(Duration::from_millis(250));
                println!("thread {} is complete", i);
                start.elapsed().as_millis() //u128类型的返回值
            }
        ));
    }

    let mut results: Vec<u128> = vec![];
    for handle in handles {
        // TODO: a struct is returned from thread::spawn, can you use it?
        // while handle.is_finished() {
        //     results.push(1);
        // }
        handle.join();
        results.push(1);
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
