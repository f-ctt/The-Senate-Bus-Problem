use std::sync::{Mutex, Arc, RwLock, Condvar};
use std::thread;
use std::time::Duration;
use std::cmp;
extern crate rand;
use rand::prelude::*;

fn enter_boardingArea(mutex_clone: Arc<Mutex<(i32)>>, id: u32) { // update the amout of riders waiting and unlock the mutex
    let mut waiting = mutex_clone.lock().unwrap();
    *waiting += 1;
    println!("RID: {}\t\t: enter: {}", id, *waiting);
}


struct Notification((Mutex<u32>, Condvar));

impl Notification {
    fn notify_all(&self, receivers: u32) {
        let &(ref lock, ref cvar) = &self.0;
        let mut notif_val = lock.lock().expect("Cannot lock");
        *notif_val = receivers; 
        cvar.notify_all();
    }

    fn wait(&self) {
        let &(ref lock, ref cvar) = &self.0;
        let mut notif_val = lock.lock().expect("Cannot lock");
        while *notif_val == 0 {
            notif_val = cvar.wait(notif_val).expect("Cannot wait");
        }
        *notif_val -= 1;
    }
}

struct Bus {
    capacity: u32,
    arrival: Notification,
    end: Notification,
}

impl Bus {
    fn send_signal_arrival(&self, waiting: u32) {
        self.arrival.notify_all(waiting);
    }

    fn wait_signal_arrival(&self) {
        self.arrival.wait();        
    }

    fn send_signal_end(&self, waiting: u32) {
        self.end.notify_all(waiting);        
    }

    fn wait_signal_end(&self) {
        self.end.wait();
    }
}
//TODO: build threads before spawning them and name them
fn main() {

    let waiting = Arc::new(Mutex::new(0_i32));
    let allAboard = Arc::new((Mutex::new(0), Condvar::new())); // Bus waits til all riders get in
    let bus = Arc::new(Bus{ capacity: 5, 
        arrival: Notification((Mutex::new(0), Condvar::new())), 
        end: Notification((Mutex::new(0), Condvar::new())) 
        });
    
    let mut handles = vec![];
    const MAX_RIDERS: u32 = 10;

    let waiting_clone = waiting.clone();
    let allAboard_clone = allAboard.clone();
    let bus_c = bus.clone();

    let handle = thread::spawn( move || {   // BUS
        println!("BUS\t\t: start");
        let mut counter: u32 = 0;
        
        while counter < MAX_RIDERS {
            let mut waiting = waiting_clone.lock().unwrap();
            println!("BUS\t\t: arrival");
            let min = cmp::min(*waiting, bus_c.capacity as i32) as u32; // TODO: capacity from argv
            *waiting = cmp::max(*waiting - bus_c.capacity as i32, 0);
            counter += min;

            bus_c.send_signal_arrival(min);

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

            bus_c.send_signal_end(min);

            println!("BUS\t\t: end");
        }
        println!("BUS\t\t: finish");
    });
    handles.push(handle);
    //TODO: make min a shared var    
    for x in 0 .. MAX_RIDERS {

        let all_aboard_c = allAboard.clone();
        let waiting_clone = waiting.clone();
        let bus_c = bus.clone();

        let r: u8 = random();
        thread::sleep(Duration::from_millis(r as u64));

        let handle = thread::spawn(move || {    //RIDERS
            let id = x + 1;
            println!("RID: {}\t\t: start", id);

            enter_boardingArea(waiting_clone, id);

            bus_c.wait_signal_arrival();

            let &(ref lock, ref cvar) = &*all_aboard_c;
            let mut all_aboard = lock.lock().unwrap();
            *all_aboard += 1;
            cvar.notify_one();  // notify the bus a rider has boarded
            println!("RID: {}\t\t: boarding: {}", id, *all_aboard);
            drop(all_aboard);

            bus_c.wait_signal_end();

            println!("RID: {}\t\t: finish", id);
        });
        handles.push(handle);
    }


    for h in handles {
        h.join().unwrap();
    }
}
