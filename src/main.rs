use garbled_circuits::{
    garble::{Garbled, SimpleGarbledGate},
    gate::{Bit, NandGate},
};

fn main() {
    let secret = 42;
    let nand = NandGate;

    println!(
        "\n\n\nAlice and Bob each have a bit and want to to compute the result of ANDing their bits"
    );
    println!("{:?}", nand);
    println!("However, they do not want to share their bits with each other.");
    println!();
    println!(
        "Alice has some secret, which she uses to encrypt the inputs and outputs of the gate.\n"
    );
    println!("For each input and output bits, Alice encrypts them with keys generated from here secret key.\n");

    let garbled_nand = SimpleGarbledGate::new(secret, nand);
    let garbled_and_table = garbled_nand.compute_garble_table();

    println!("The garbled AND table now looks like this:");
    println!("{}", garbled_and_table);

    let alice_bit = Bit::Zero;

    println!(
        "Alice now fetches the partially applied table corresponding to her bit {}",
        alice_bit
    );

    let partial_applied_table = garbled_and_table.get_partial_applied_table(alice_bit);

    println!("{:?}", partial_applied_table);

    println!();
    println!("Now we need to do oblivious transfer. We'll do commutative encryption where, for each possible input secret in an order, Alice encrypts and sends to Bob, Ea(s_1), Ea(s_2)..., Ea(s_n).");
    println!(
        "\nAnd bob chooses the index he wants and encrypts(Eb(Ea(s_i))) that an sends back to Alice."
    );
    println!("\nAlice then decrypts the value bob sent and then sends the result to Bob, Eb(s_i).");
    println!("\nBob then decrypts it to get the key to the row he is looking for, s_i.");
}
