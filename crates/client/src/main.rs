use loom_core::Health;

fn main() {
    let h = Health {
        status: "from client".into(),
    };

    println!("{:?}", h);
}
