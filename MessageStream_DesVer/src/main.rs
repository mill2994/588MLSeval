use deevee::*;
use rand_core::OsRng;
use std::time::{Instant, Duration};


// Designated Verifier Benchmark Code
// Move the timer around the code to test
fn main() {
    let mut total_duration = Duration::new(0, 0);

    // Repeats the code 1000 times 
    const NUM_REPETITIONS: usize = 1000;
    for _ in 0..NUM_REPETITIONS {
        
        // Start timer
        let start = Instant::now();

        let (privA, pubA) = generate_keypair(&mut OsRng);

        // Stop the timer
        let duration = start.elapsed();
        total_duration += duration;
        
        // Remaining gen key algorithms
        let (privB, pubB) = generate_keypair(&mut OsRng);
        let (privC, pubC) = generate_keypair(&mut OsRng);

        
        // Sign algorithm
        let sig = privA.sign(&mut OsRng, &pubB, b"I like cats");

        
        // Verification algorithms

        // The signature verifies, because the designee matches
        assert!(privB.verify(&pubA, b"I like cats", &sig));

        // If we change the message, verification fails
        assert!(!privB.verify(&pubA, b"I don't like cats", &sig));
    
        // The signer won't verify with a different signer either
        assert!(!privB.verify(&pubC, b"I like cats", &sig));

        // The wrong verifier can't validate the signature either
        assert!(!privC.verify(&pubA, b"I like cats", &sig));

        
        // Forge algorithm

        // Finally, the verifier can forge a valid signature for themselves
        let forged = privB.forge(&mut OsRng, &pubA, b"I don't like cats");


        assert!(privB.verify(&pubA, b"I don't like cats", &forged));
    }

    // Calculate the average time
    let average_duration = total_duration / NUM_REPETITIONS as u32;

    // Print the average time
    println!("Average time elapsed: {:?}", average_duration);
}
