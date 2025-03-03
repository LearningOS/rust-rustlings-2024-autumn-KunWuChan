// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.


use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

/*
权限转移方式
不可变共享：Arc<T>：
    Arc（Atomic Reference Counting）允许多线程共享只读数据。
    每个线程通过 Arc::clone 获得一个引用计数指针，指向同一块内存。
    但 Arc<T> 内的 T 是不可变的，不能直接修改。
可变共享：Arc<Mutex<T>> 或 Arc<RwLock<T>>：
    要修改共享变量，需搭配同步原语：
        Mutex：互斥锁，保证一次只有一个线程修改。
        RwLock：读写锁，支持多读单写。
    Arc 提供共享，Mutex 提供修改权限的转移。

*/
fn main() {
    let status = Arc::new(Mutex::new(JobStatus{ jobs_completed: 0 })); //加锁，允许线程安全修改
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status); //只读引用
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value

            // 获取锁，以更新共享数据
            let mut status = status_shared.lock().unwrap();
            // 线程安全，只读引用
            println!("update jobs_completed{}",status.jobs_completed);
            status.jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
        // anything interesting in the output? Do you have to 'join' on all the
        // handles?
    }
    let status = status.lock().unwrap();
    println!("jobs completed {}", status.jobs_completed);
}
