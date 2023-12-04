use futures::future::{self, AbortHandle, Abortable};
use std::time::Duration;
use tokio::time;

struct S {
    i: i32,
}

impl S {
    fn new(i: i32) -> Self {
        println!("Creating S {}", i);
        S { i }
    }
}

impl Drop for S {
    fn drop(&mut self) {
        println!("Dropping S {}", self.i);
    }
}

#[tokio::main]
async fn main() {
    let create_s = async {
        let s = S::new(42);
        // time::delay_for(Duration::from_millis(200)).await;
        // time::sleep(Duration::from_secs(5));
        loop {
            println!("Creating {} done", s.i);
            time::sleep(Duration::from_secs(1)).await;
        }

        
    };
    let (abort_handle, abort_registration) = AbortHandle::new_pair();
    let create_s = Abortable::new(create_s, abort_registration);

    let abort_s = async move {
        // time::delay_for(Duration::from_millis(100)).await;
        time::sleep(Duration::from_secs(5)).await;

        abort_handle.abort();
    };

    let c = tokio::spawn(create_s);
    let a = tokio::spawn(abort_s);

    let (c, a) = future::join(c, a).await;

    println!("{:?}, {:?}", c, a);
}