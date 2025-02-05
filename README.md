# Data Race in Rust using Raw Pointers

This repository demonstrates a data race in Rust that can occur when using raw pointers without proper synchronization. Data races are a common source of concurrency bugs, causing unpredictable behavior and potential crashes.

## The Bug
The `bug.rs` file contains code that demonstrates a data race.  The program uses raw pointers to manipulate a vector. Without proper synchronization mechanisms, concurrent access to the vector can lead to unexpected results.

## The Solution
The `bugSolution.rs` file shows how to fix this data race using appropriate synchronization techniques. This example uses mutexes to protect the shared vector, ensuring that only one thread can modify it at any given time.

## How to run
1. Clone this repository
2. Navigate to the directory
3. Compile and run the code: `cargo run --example bug` and `cargo run --example bugSolution`

## Key Concepts
* **Data Races:** Simultaneous access to shared resources (like variables) without synchronization mechanisms, leading to unpredictable behavior.
* **Raw Pointers:** Unsafe pointers that require careful management and can lead to memory-related errors if not handled correctly.
* **Mutexes:** Mutual exclusion locks that protect access to shared resources, preventing data races.