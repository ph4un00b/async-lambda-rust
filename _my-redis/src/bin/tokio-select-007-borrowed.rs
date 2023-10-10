use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    let mut out = String::new();

    tokio::spawn(async move {
        let _ = tx1.send("one");
        let _ = tx2.send("two");
    });
    /*
     * When it comes to each branch's <handler>,
     * select! guarantees that only a single <handler> runs.
     * Because of this, each <handler> may mutably borrow the same data.
     */
    tokio::select! {
        _ = rx1 => {
            out.push_str("rx1 completed");
        }
        _ = rx2 => {
            out.push_str("rx2 completed");
        }
    }

    println!("{}", out);
}
