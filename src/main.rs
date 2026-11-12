use std::io;

fn main() {
    println!("tebak angka?");
    let tebakan = rand::random::<u32>() % 100 + 1;
    println!("angka yang benar adalah: {}", tebakan);

    let mut tebak = String::new();

    io::stdin()
        .read_line(&mut tebak)
        .expect("salah ea");

    println!("kamu menebak: {}", tebak);
}
