use std::thread;
use std::time::Duration;

// If main subprocess ends, then other subprocesses should end too
fn main() {
	let handle = thread::spawn(|| {
    	for i in 1..10 {
        	println!("Hello {} from first thread", i);
        	thread::sleep(Duration::from_millis(1));
    	}
	});

	handle.join().unwrap();

	for i in 1..5 {
    	println!("Hello {} from second thread", i);
    	thread::sleep(Duration::from_millis(1));
	}
    
	for i in 1..4 {
    	println!("Hello {} from third thread", i);
    	thread::sleep(Duration::from_millis(1));
	}
}
