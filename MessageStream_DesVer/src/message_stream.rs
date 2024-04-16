use std::thread;
use std::time::{Instant, Duration};

// Message Streams benchmark tests
fn main() {
    
    let experiment = 4; 
    // 1 = single stream
    // 2 = multi stream, one processor, 100% priority
    // 3 = multi stream, one processor, divided priority
    // 4 = multi stream, multi processor
    
    if experiment == 1 {
        // Single stream, no priority or priotizing
        println!("Single stream, no priority or priotizing");
        
        let mut singleStreamMessages: Vec<String> = Vec::new();
    
        for _ in 0..100 {
            singleStreamMessages.push(String::from("bulk message"));
        }
        
        for _ in 0..10 {
            singleStreamMessages.push(String::from("urgent message"));
        }
    
        // Start timer
        let start = Instant::now();
        
        
        
        while let Some(msg) = singleStreamMessages.get(0).cloned() {
            singleStreamMessages.remove(0);
            thread::sleep(Duration::from_millis(500));
            
            
            let elapsed = start.elapsed();
            // Print the elapsed time
            println!("Time elapsed: {:?}, msg type: {}", elapsed, msg);
        }
        

        // Stop the timer
        let duration = start.elapsed();
    
        // Print the elapsed time
        println!("Total time elapsed: {:?}", duration);
    } 
    
    
    
    
    // Multi stream, one processor
    let mut bulkStream: Vec<String> = Vec::new();
    let mut urgentStream: Vec<String> = Vec::new();
    
    if experiment == 2 {
        for _ in 0..100 {
            bulkStream.push(String::from("bulk message"));
        }
        
        for _ in 0..10 {
            urgentStream.push(String::from("urgent message"));
        }
    
    
        // Multi stream, one processor, urgent has all priority
        println!("Multi stream, one processor, urgent has all priority");
        
        // Start timer
        let start = Instant::now();
        
        while !urgentStream.is_empty() || !bulkStream.is_empty() {
            if !urgentStream.is_empty() {
                // If urgent_messages is not empty, remove one urgent message
                if let Some(urgMsg) = urgentStream.pop() {
                    println!("Removed urgent message: {}", urgMsg);
                    thread::sleep(Duration::from_millis(500));
                    let elapsed = start.elapsed();
                    // Print the elapsed time
                    println!("Time elapsed: {:?}, msg type: {}", elapsed, urgMsg);
                }
            } else {
                // If urgent_messages is empty, remove one bulk message
                if let Some(bulkMsg) = bulkStream.pop() {
                    println!("Removed bulk message: {}", bulkMsg);
                    thread::sleep(Duration::from_millis(500));
                    let elapsed = start.elapsed();
                    // Print the elapsed time
                    println!("Time elapsed: {:?}, msg type: {}", elapsed, bulkMsg);
                }
            }
        }
        
        // Stop the timer
        let duration = start.elapsed();
    
        // Print the elapsed time
        println!("Total time elapsed: {:?}", duration);
        
        
    }
    
    if experiment == 3 {
        for _ in 0..100 {
            bulkStream.push(String::from("bulk message"));
        }
        
        for _ in 0..10 {
            urgentStream.push(String::from("urgent message"));
        }
        
        // If urgent/higher priority messages/data are more frequent than expected, or you want some divided priority... then allow for a 2:1 ratio. Otherwise, urgent messages may never end and no bulk get processed
        // Multi stream, one processor, divided priority
        println!("Multi stream, one processor, divided priority");
        
        // Start timer
        let start = Instant::now();
        
        while !urgentStream.is_empty() || !bulkStream.is_empty() {
            // Process one urgent message if available
            if let Some(urgMsg) = urgentStream.pop() {
                println!("Removed urgent message: {}", urgMsg);
                thread::sleep(Duration::from_millis(500));
                let elapsed = start.elapsed();
                // Print the elapsed time
                println!("Time elapsed: {:?}, msg type: {}", elapsed, urgMsg);
            }
    
            // Process two bulk messages if available
            for _ in 0..2 {
                if let Some(bulkMsg) = bulkStream.pop() {
                    println!("Removed bulk message: {}", bulkMsg);
                    thread::sleep(Duration::from_millis(500));
                    let elapsed = start.elapsed();
                    // Print the elapsed time
                    println!("Time elapsed: {:?}, msg type: {}", elapsed, bulkMsg);
                } else {
                    // Break the loop if there are no more bulk messages
                    break;
                }
            }
        }
        
        // End timer
        let duration = start.elapsed();
    
        // Print the elapsed time
        println!("Total time elapsed: {:?}", duration);
    }
    
    if experiment == 4 {
        // Multi stream, multi processor
        println!("Multi stream, multi processor");
        
        for _ in 0..100 {
            bulkStream.push(String::from("bulk message"));
        }
        
        for _ in 0..10 {
            urgentStream.push(String::from("urgent message"));
        }
        
        // Start timer
        let start = Instant::now();
        
        let handle1 = thread::spawn(move || {
            while let Some(msgU) = urgentStream.pop() {
                //println!("Removed urgent message: {}", msgU);
                thread::sleep(Duration::from_millis(500));
                let elapsed = start.elapsed();
                // Print the elapsed time
                println!("Time elapsed: {:?}, msg type: {}", elapsed, msgU);
            }
        });
    
        let handle2 = thread::spawn(move || {
            while let Some(msgB) = bulkStream.pop() {
                //println!("Removed bulk message: {}", msgB);
                thread::sleep(Duration::from_millis(500));
                let elapsed = start.elapsed();
                // Print the elapsed time
                println!("Time elapsed: {:?}, msg type: {}", elapsed, msgB);
            }
        });
    
        handle1.join().unwrap();
        handle2.join().unwrap();
        
        // End timer
        let duration = start.elapsed();
    
        // Print the elapsed time
        println!("Total time elapsed: {:?}", duration);
    }
    
    
    
    // Check to see how many of each type of message are proccesed in 1 minute
    
    // Check to see how long it takes to proccess n of each message type
    
}