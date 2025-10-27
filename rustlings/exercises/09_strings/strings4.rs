// Calls of this function should be replaced with calls of `string_slice` or `string`.
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// ✅ Fixed: Choose correct function for each case
fn main() {
    string_slice("blue"); // &str literal

    string("red".to_string()); // String

    string(String::from("hi")); // String

    string("rust is fun!".to_owned()); // String

    string("nice weather".into()); // String (From<&str> → String)

    string(format!("Interpolation {}", "Station")); // String

    // Byte slice of a String → &str
    string_slice(&String::from("abc")[0..1]);

    // trim() returns &str
    string_slice("  hello there ".trim());

    // replace() returns String
    string("Happy Monday!".replace("Mon", "Tues"));

    // to_lowercase() returns String
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
