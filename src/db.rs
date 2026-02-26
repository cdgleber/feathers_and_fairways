use petname::Generator;
// use rand::Rng; // Trait needs to be in scope for `generate`.
pub fn generate_access_key() -> String {
    //const CHARSET: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
    let mut rng = rand::thread_rng();
    let name = petname::Petnames::default()
        .generate(&mut rng, 3, "-")
        .expect("no names");
    name
}
