use std::time::Duration;

use tokio;

#[tokio::main]
async fn main() {
    let task1 = tokio::spawn(async {
        for i in 0..10 {
            println!("task1 frame{i}");
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    });

    let task2 = tokio::spawn(async {
        for i in 0..10 {
            println!("task2 frame{i}");
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    });

    // Wait for both tasks to complete
    task1.await.unwrap();
    task2.await.unwrap();
}