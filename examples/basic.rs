use slip::slip;

fn main() {
    // Do not forget to provide the 16 bytes hexadecimal SLIP_KEY environment variable at build time.
    // Otherwise, the slip macro will have no effect. SLIP_KEY is used as the unique key to encrypt the strings.
    // You should of course not include it in your production builds.

    // Please note that the macro will only error out about missing keys in release mode or if the
    // "allow-no-encryption" feature is enable, so you can debug free of hassle.

    let test_value: Option<bool> = None; // Let's pretend some error-handling logic gets triggered.

    test_value.expect(slip!("failed to do whatever thing we were meant to do"));
}