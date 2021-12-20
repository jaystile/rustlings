// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 22 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. Because of the difference between the
// spawned threads' sleep time, and the waiting threads sleep time, when you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;


// TODO:
// This could use a little feedback.
// Most lines changed. There are handles, Mutex, Probably do handles first then do a mutex in threads2


struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            let res = status_shared.lock();
            match res {
                Ok(mut ok) => ok.jobs_completed += 1,
                Err(err) => panic!("Could not acquire lock! {}", err)
            }
        });
    }
    while status.lock().unwrap().jobs_completed < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}
