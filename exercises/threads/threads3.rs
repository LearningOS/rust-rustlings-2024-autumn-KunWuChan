// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.


use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: Arc<mpsc::Sender<u32>>) -> Vec<thread::JoinHandle<()>> {
    let qc = Arc::new(q);
    let qc1 = Arc::clone(&qc);
    let qc2 = Arc::clone(&qc);

    let tx1 = Arc::clone(&tx);
    let mut handles = Vec::new();
    //线程要求所有传入的数据拥有 'static 生命周期（即在程序整个运行期间有效），确保线程不会引用已销毁的数据（避免悬垂指针）
    handles.push(thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            tx1.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    }));

    let tx2 = Arc::clone(&tx);
    handles.push(thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            tx2.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    }));
    handles
}
//一个接收端rx,两个子进程tx发送，统计总的收到的数据
fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    let tx = Arc::new(tx);
    let handles = send_tx(queue, Arc::clone(&tx));
    // 丢弃主线程的 tx，确保通道能在所有发送完成后关闭
    drop(tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}
