use garbled_circuits::{AndGate, Garbled, GarbledGate, Gate};

fn main() {
    let and = AndGate;
    let secret = 42;
    println!("{}", and.as_str());
    let garbled_and = GarbledGate::new(secret, and.clone());
    println!(
        "{}",
        <GarbledGate<AndGate> as Garbled>::compute_garble_table(secret, &and)
    );
}
