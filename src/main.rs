use crypter;

fn main() {
    let pass = include!("./encryption");
    let payload = "mega ultra safe payload";

    let encrypted = crypter::encrypt(pass.as_bytes(), payload.as_bytes()).expect("Failed to encrypt");

    println!("{:?}", &encrypted);
    let decrypted = crypter::decrypt(pass.as_bytes(), &encrypted).expect("Failed to decrypt");
    println!("{}", String::from_utf8(decrypted).expect("Invalid decrypted string"));
}
