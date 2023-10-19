use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Queue {
    length: usize,
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

fn send_tx(q: Arc<Queue>, tx: Arc<Mutex<mpsc::Sender<u32>>>) {
    let qc1 = Arc::clone(&q);
    let qc2 = Arc::clone(&q);

    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            tx.lock().unwrap().send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for val in &qc2.second_half {
        println!("sending {:?}", val);
        tx.lock().unwrap().send(*val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Arc::new(Queue::new());
    let queue_length = queue.length;

    let tx_mutex = Arc::new(Mutex::new(tx));

    send_tx(Arc::clone(&queue), Arc::clone(&tx_mutex));

    let mut total_received: usize = 0;
    for _ in rx {
        total_received += 1;
        if total_received == queue_length {
            break;
        }
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length);

    drop(tx_mutex);
}
