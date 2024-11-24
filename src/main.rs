use garbled_circuits::{
    garble::{Garbled, SimpleGarbledGate},
    gate::{AndGate, Bit, NandGate},
};

fn main() {
    let secret = 42;
    let and = AndGate;

    println!(
        "\n\n\nAlice and Bob each have a bit and want to to compute the result of ANDing their bits"
    );
    println!("{:?}", and);
    println!("However, they do not want to share their bits with each other.");
    println!();
    println!(
        "Alice has some secret, which she uses to encrypt the inputs and outputs of the gate.\n"
    );
    println!("For each input and output bits, Alice encrypts them with keys generated from here secret key.\n");
    let garbled_and_table =
        <SimpleGarbledGate<NandGate> as Garbled>::compute_garble_table(secret, &and);
    println!("The garbled AND table now looks like this:");
    println!("{}", garbled_and_table);

    let alice_bit = Bit::Zero;
    println!(
        "Alice now fetches the rows of table corresponding to her bit {}",
        alice_bit
    );
    println!("{:?}", garbled_and_table.get_table_for_input(alice_bit));
}
