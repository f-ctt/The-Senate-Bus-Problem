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

struct Bus {
    capacity: u32,
    arrival: Mutex<i32>,
    arrival_cond: Condvar,
}

impl Bus {
    fn send_arrival_signal(&self, waiting: i32) {
        let mut arr_s = self.arrival.lock().expect("Cannot lock - send_arrival");
        *arr_s = waiting;
        self.arrival_cond.notify_all();
    }

    fn wait_arrival_signal(&self) {
        let mut arr_s = self.arrival.lock().expect("Cannot lock - wait_arrival");
        while *arr_s == 0 {
            arr_s = self.arrival_cond.wait(arr_s).expect("Cannot wait - arrival_cond");
        }
        *arr_s -= 1; // decrease the amout of waiting receivers
    }
}
//TODO: build threads before spawning them and name them
fn main() {

    let waiting = Arc::new(Mutex::new(0_i32));
    let allAboard = Arc::new((Mutex::new(0), Condvar::new())); // Bus waits til all riders get in
    let bus_signal = Arc::new((Mutex::new(0_i32), Condvar::new())); // Bus arrival
    let bus_end = Arc::new((Mutex::new(0_i32), Condvar::new())); // Bus end
    let bus = Arc::new(Bus { capacity: 5, arrival: Mutex::new(0), arrival_cond: Condvar::new()});
    
    let mut handles = vec![];
    const MAX_RIDERS: i32 = 10;

    let waiting_clone = waiting.clone();
    let bus_signal_clone = bus_signal.clone();
    let allAboard_clone = allAboard.clone();
    let bus_end_clone = bus_end.clone();
    let bus_c = bus.clone();

    let handle = thread::spawn( move || {   // BUS
        println!("BUS\t\t: start");
        let mut counter: i32 = 0;
        
        while counter < MAX_RIDERS {
            let mut waiting = waiting_clone.lock().unwrap();
            println!("BUS\t\t: arrival");
            let min = cmp::min(*waiting, bus_c.capacity as i32); // TODO: capacity from argv
            *waiting = cmp::max(*waiting - bus_c.capacity as i32, 0);
            counter += min;

            bus_c.send_arrival_signal(min);

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
    for x in 0 .. MAX_RIDERS {

        let all_aboard_c = allAboard.clone();
        let waiting_clone = waiting.clone();
        let bus_signal_clone = bus_signal.clone();
        let bus_end_clone = bus_end.clone();
        let bus_c = bus.clone();

        let r: u8 = random();
        thread::sleep(Duration::from_millis(r as u64));

        let handle = thread::spawn(move || {    //RIDERS
            let id = x + 1;
            println!("RIDER: {}\t: start", id);

            enter_boardingArea(waiting_clone);
            println!("RIDER: {}\t: enter", id);

            bus_c.wait_arrival_signal();

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
