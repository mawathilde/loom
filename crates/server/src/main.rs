use loom_core::Health;

#[tokio::main]
async fn main() {
    let health = Health {
        status: "ok".into(),
    };

    println!("Server health: {:?}", health);
}
