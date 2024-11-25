#![feature(generic_const_exprs)]
use std::io;

use garbled_circuits::{
    encryption::{EncryptionScheme, SimpleEncryptionScheme},
    garble::{Garbled, SimpleGarbledGate},
    gate::{Bit, ANDGATE},
};

fn pause() {
    let mut inp = String::new();
    println!("Press Enter to continue...");
    io::stdin()
        .read_line(&mut inp) // Read input from the user
        .expect("Failed to read input");
}

fn main() {
    let alice_secret = 42;
    let bob_secret = 24;

    println!(
        "\n\n\nAlice and Bob each have a bit and want to to compute the result of ANDing their bits"
    );
    // println!("{:?}\n", ANDGATE);
    println!("However, they do not want to share their bits with each other.");
    println!();
    println!(
        "Alice has some secret, which she uses to encrypt the inputs and outputs of the gate.\n"
    );
    println!("For each input and output bits, Alice encrypts them with keys generated from here secret key.\n");

    let garbled_nand = SimpleGarbledGate::new(alice_secret, ANDGATE);
    let garbled_and_table = garbled_nand.compute_garble_table();

    println!("The garbled table now looks like this:");
    println!("{}", garbled_and_table);

    let alice_bit = Bit::Zero;

    println!(
        "Alice now fetches the partially applied table corresponding to her bit {}",
        alice_bit
    );
    pause();

    let partial_applied_table = garbled_and_table.get_partial_applied_table(alice_bit);

    println!("{:?}", partial_applied_table);
    println!();
    pause();

    println!("Now we need to do oblivious transfer. We'll do commutative encryption where, for each possible input secret in an order, Alice encrypts and sends to Bob, Ea(s_1), Ea(s_2)..., Ea(s_n).");
    println!(
        "\nAnd bob chooses the index he wants and encrypts(Eb(Ea(s_i))) that an sends back to Alice."
    );
    println!("\nAlice then decrypts the value bob sent and then sends the result to Bob, Eb(s_i).");
    println!("\nBob then decrypts it to get the key to the row he is looking for, s_i.");

    println!("\n\nSTART!");
    let garbled_to_bob = partial_applied_table.hash_outputs;

    let alice_enc = SimpleEncryptionScheme(alice_secret);
    let bob_enc = SimpleEncryptionScheme(bob_secret);

    let bob_received_inputs: Vec<_> = partial_applied_table
        .inps_sorted
        .iter()
        .map(|(a, b)| (alice_enc.encrypt(*a), alice_enc.encrypt(*b)))
        .collect();

    println!("Bob receives garbled circuit eval:\n{:?}", garbled_to_bob);
    pause();
    println!(
        "Bob receives encrypted gate passwords:\n{:?}",
        bob_received_inputs
    );
    println!(
        "Actual gate passwords:\n{:?}",
        partial_applied_table.inps_sorted
    );
    pause();

    let bob_bit = Bit::One;
    let index: u64 = bob_bit.into();
    println!(
        "Bob's bit is {:?} which corresponds to {}th password",
        bob_bit, index
    );
    pause();
    let bob_input = bob_received_inputs[index as usize];
    let bob_encrypted = (bob_enc.encrypt(bob_input.0), bob_enc.encrypt(bob_input.1));

    println!(
        "Bob encrypts his desired input: {:?} and sends to Alice",
        bob_encrypted,
    );
    pause();

    let alice_decrypted = (
        alice_enc.decrypt(bob_encrypted.0),
        alice_enc.decrypt(bob_encrypted.1),
    );

    println!(
        "Alice decrypts it to get {:?} and sends back to Bob",
        alice_decrypted
    );
    pause();

    let bob_decrypted = (
        bob_enc.decrypt(alice_decrypted.0),
        bob_enc.decrypt(alice_decrypted.1),
    );

    println!("Bob decrypts it to get the password {:?}", bob_decrypted);
    pause();

    let concatenated =
        <SimpleGarbledGate<2> as Garbled<2>>::concat(bob_decrypted.0, bob_decrypted.1);
    let hash = <SimpleGarbledGate<2> as Garbled<2>>::hash(&concatenated);
    println!("Bob hashes the password to get {:?} which is the key to the garble table he received before.", hash);
    println!(
        "Bob uses the hash to get the result: {:?}",
        garbled_to_bob.get(&hash)
    );
}
