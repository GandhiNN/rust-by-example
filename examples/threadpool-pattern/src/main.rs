use threadpool_pattern::ThreadPool;

fn main() {
    let pool = ThreadPool::new(6);
    for i in 0..6 {
        pool.execute(move |message| {
            println!("Task: {} prints {}", i, message);
        });
    }
    pool.join();
}
