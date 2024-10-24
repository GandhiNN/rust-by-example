use std::time::Duration;

async fn std_print() {
    std::thread::sleep(Duration::from_secs(5));
    println!("Five seconds later...");
}

async fn sleep_then_print_without_await(timer: i32) {
    println!("Start timer {}.", timer);
    std::thread::sleep(Duration::from_secs(1));
    println!("Timer {} done.", timer);
}

async fn sleep_then_print_with_await(timer: i32) {
    println!("Start timer {}.", timer);
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("Timer {} done.", timer);
}

async fn tokio_spawn_blocking() {
    let blocking_task = tokio::task::spawn_blocking(|| {
        // This is running on a thread where blocking is fine
        println!("Inside spawn_blocking");
    });
    // Wait for the blocking task
    blocking_task.await.unwrap();
}

async fn parallel_sum_rayon_to_tokio(numvec: Vec<i32>) -> i32 {
    let (send, recv) = tokio::sync::oneshot::channel();

    // Spawn a task on Rayon
    rayon::spawn(move || {
        // Expensive computation
        let mut sum = 0;
        for num in numvec {
            sum += num;
        }
        // Send the result back to tokio
        let _ = send.send(sum);
    });
    // Wait for the Rayon task
    recv.await.expect("Panic in rayon::spawn")
}

#[tokio::main]
async fn main() {
    println!("Hello world!");
    // Standard Print
    std_print();

    // Sleep and then print result without awaiting the future
    // Use join to make sure the thread finished
    tokio::join!(
        sleep_then_print_without_await(1),
        sleep_then_print_without_await(2),
    );

    // Sleep and then print with awaiting the future
    // Use join to make sure the thread finished
    tokio::join!(
        sleep_then_print_with_await(1),
        sleep_then_print_with_await(2),
    );

    // Spawn blocking task in tokio
    tokio_spawn_blocking();

    // Compute with Rayon and send the result into tokio channel
    let nums = vec![1; 1024 * 1024]; // huge vector of i32 element
    println!("{}", parallel_sum_rayon_to_tokio(nums).await);
}
