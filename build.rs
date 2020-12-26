use std::fs::write;

const KEYS_FILE: &str = "src/keys.rs";

fn main() {
    // We write the API key using the build script for safety and security.
    // This way, we can put `keys.rs` in the .gitignore so no one will see our API key!
    write(KEYS_FILE, format!("pub const GOVEE_API_KEY: &str = {:?};", env!("GOVEE_API_KEY", "Set `GOVEE_API_KEY` to your govee api key.")))
        .expect("Could not write keys to file");
}