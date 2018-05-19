use std::sync::{Mutex, Arc, RwLock, Condvar};
use std::thread;
use std::time::Duration;
use std::cmp;
extern crate rand;
use rand::prelude::*;

fn enter_boardingArea(mutex_clone: Arc<Mutex<(i32)>>) { // update the amout of riders waiting and unlock the mutex
    let mut waiting = mutex_clone.lock().unwrap();
    *waiting += 1;
}

fn wait_for_signal(mutex_clone: Arc<((Mutex<(i32)>, Condvar<>))>) {
    let &(ref lock, ref cvar) = &*mutex_clone;
    let mut signal = lock.lock().unwrap();
    while *signal == 0 {
        signal = cvar.wait(signal).unwrap();
    }
    *signal -= 1;
}

fn main() {

    let waiting = Arc::new(Mutex::new(0_i32));
    let allAboard = Arc::new((Mutex::new(0), Condvar::new())); // Bus waits til all riders get in
    let bus_signal = Arc::new((Mutex::new(0_i32), Condvar::new())); // Bus arrival
    let bus_end = Arc::new((Mutex::new(0_i32), Condvar::new())); // Bus end
    
    let mut handles = vec![];

    let waiting_clone = waiting.clone();
    let bus_signal_clone = bus_signal.clone();
    let allAboard_clone = allAboard.clone();
    let bus_end_clone = bus_end.clone();

    let handle = thread::spawn( move || {   // BUS
        println!("BUS\t\t: start");
        let mut counter = 0;
        
        while counter < 10 {
            let mut waiting = waiting_clone.lock().unwrap();
            println!("BUS\t\t: arrival");
            let min = cmp::min(*waiting, 5); // TODO: capacity from argv
            *waiting = cmp::max(*waiting - 5, 0);
            counter += min;

            let &(ref lock, ref cvar) = &*bus_signal_clone;
            let mut bus_signal = lock.lock().unwrap();
            *bus_signal = min;
            cvar.notify_all();
            drop(bus_signal);


            let &(ref lock, ref cvar) = &*allAboard_clone;
            let mut all_aboard = lock.lock().unwrap();
            while *all_aboard != min {  // wait for all riders to get in
                all_aboard = cvar.wait(all_aboard).unwrap();
            }
            *all_aboard = 0;
            
            drop(waiting);  // unclock the mutex

            println!("BUS\t\t: depart");
            let r: u8 = random();
            thread::sleep(Duration::from_millis(r as u64));

            let &(ref lock_2, ref cvar_2) = &*bus_end_clone;
            let mut bus_end = lock_2.lock().unwrap();
            *bus_end = min;
            cvar_2.notify_all();

            println!("BUS\t\t: end");
        }
        println!("BUS\t\t: finish");
    });
    handles.push(handle);
    //TODO: make min a shared var    
    let all_aboard_c = allAboard.clone();
    let waiting_clone = waiting.clone();
    let bus_signal_clone = bus_signal.clone();
    let bus_end_clone = bus_end.clone();
    for x in 0 .. 10 {


        let r: u8 = random();
        thread::sleep(Duration::from_millis(r as u64));

        let handle = thread::spawn(move || {    //RIDERS
            let id = x;
            println!("RIDER: {}\t: start", id);

            enter_boardingArea(waiting_clone);
            println!("RIDER: {}\t: enter", id);

            wait_for_signal(bus_signal_clone);

            let &(ref lock, ref cvar) = &*all_aboard_c;
            let mut all_aboard = lock.lock().unwrap();
            *all_aboard += 1;
            cvar.notify_one();  // notify the bus a rider has boarded
            drop(all_aboard);
            println!("RIDER: {}\t: boarding", id);

            let &(ref lock, ref cvar) = &*bus_end_clone;
            let mut bus_end = lock.lock().unwrap();
            while *bus_end == 0{   // wait for the bus to end road
                bus_end = cvar.wait(bus_end).unwrap();
            }
            *bus_end -= 1;
            println!("RIDER: {}\t: finish", id);
        });
        handles.push(handle);
    }


    for h in handles {
        h.join().unwrap();
    }
}
