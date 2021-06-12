// program that has a thread to generate values ​​and send them through a channel, 
// and another thread that will receive the values ​​and print them. We will send 
// simple values ​​between threads using a channel to illustrate the function.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {

	let (tx, rx) = mpsc::channel();

	let tx1 = mpsc::Sender::clone(&tx);
	thread::spawn(move || {
    	let vals = vec![
        	String::from("Hello"),
        	String::from("from"),
        	String::from("the"),
        	String::from("first thread"),
    	];

    	for val in vals {
        	tx1.send(val).unwrap();
        	thread::sleep(Duration::from_secs(1));
    	}
	});

	thread::spawn(move || {
    	let vals = vec![
        	String::from("Let's"),
        	String::from("how this "),
        	String::from("works"),
        	String::from("thread 2"),
    	];

    	for val in vals {
        	tx.send(val).unwrap();
        	thread::sleep(Duration::from_secs(1));
    	}
	});

	for received in rx {
    	println!("Get message {}", received);
	}

}
