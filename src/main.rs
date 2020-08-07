use std::thread; //thread lib
use std::time::Duration; // time lib

// Multi threading
fn get_sites(){
    let thread_one = thread::spawn(|| {
        thread::sleep(Duration::from_millis(500));
        println!("Thread One!");        
    });
    let thread_two = thread::spawn(||{
        println!("Thread Two!");
    });

    thread_one.join().expect("Thread One Panicked! ");
    thread_two.join().expect("Thread Two Panicked! ");
}

fn main() {
    get_sites();
}
