# slip

[![Crates.io][crates-badge]][crates]
[![docs page][docs-badge]][docs]
[![MIT][license-badge]][license]

[crates-badge]: https://img.shields.io/crates/v/slip.svg
[crates]: https://crates.io/crates/slip/

[docs-badge]: https://img.shields.io/badge/docs-website-blue.svg
[docs]: https://docs.rs/slip

[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license]: LICENSE

An hassle-free utility to encrypt error handling strings in your public binaries to protect your business logic.

**This crate DOES NOT provide general runtime obfuscation for strings. Please consider using something like [obfstr](https://github.com/CasualX/obfstr) for this purpose.**

## What does slip do?

Sometimes one wants to make sure some business logic is difficult to reverse-engineer. But they also want it to be easy to debug, even in production. However, quality error handling might leak through error strings how the program works, giving great insight to attackers. slip helps tackling this specific issue.

slip is a Rust macro that converts error strings into encrypted tokens at compile time. Attackers only get exposed to those undecipherable tokens. Then, when the error comes back to the software maintainers, they will be able to decrypt it using their secret key. The unslip utility is an easy to use, easy to automate and transparent tool to decrypt tokens, even within complex error reports.

slip works best with automated error report systems, so users never get to see any slip token and can enjoy a quality product with automated bug fixes. If any of your dependencies uses slip, you can take advantage of it as well as the key is defined globally (even if the dependency is third party).

## Installation

slip is a regular crates.io crate fetchable using cargo.

unslip is a binary. You can either **download it from the [GitHub releases](https://github.com/Moxinilian/slip/releases) for Linux and Windows (x64)**, or clone this repository and use cargo to install it on your system.

```
cargo install --path unslip
```

## Usage

You first need to generate a secret key. You can do it using unslip, but any random 16 bytes hexadecimal string will do.

```
$ unslip key
```

Then, you need to set that key to be the `SLIP_KEY` environment variable in your building environment **(please see [important considerations](#important-considerations) below)**.  
On Linux, it can be done temporarily like so (the variable will disappear once you leave the terminal):

```
$ export SLIP_KEY=<your key, without quotations>
```

On Windows, it can be done temporarily like so (the variable will disappear once you leave the terminal):

```
$ set SLIP_KEY=<your key, without quotations>
```

Once this is done, slip is ready to be used! See the [examples](examples) for how to use the macro, and [the examples section](#examples) section for how to use unslip.

## Important considerations

* Changing your SLIP_KEY value **requires a complete rebuild of your dependency tree** to make sure the change is reflected everywhere. This is due to incremental compilation, as the macro is not ran again on cold code. It is therefore recommended to avoid changing the key. You can rebuild entirely by doing `cargo clean` before building again.
* In release mode, the macro will fail building if `SLIP_KEY` is not provided. In cases where proceeding without encryption is the expected behavior for a missing key (such as in public dependencies), the `allow-no-encryption` feature should be passed to the slip dependency.
* It is not recommended to rely on the encrypted form a token to identify it as its value is volatile (it can notably change between every build).
* Of course, slip is not a miracle solution to prevent reverse engineering. Hopefully it makes it harder though.

## Help wanted

* Any feature idea or improvement are welcome! Feel free to open issues and pull requests about improvements.
* Benchmarking. While the impact of the macro should not be huge on build time (especially in development settings where it can do nothing if no key is provided), I would really like to have an insight on wether it is worth finding more performant solutions than what I currently have set up. Really the best way to know would be to analyze its effect on a somewhat large codebase. If you happen to have one, it would be really appreciated if you shared your results!
* This is my very first procedural macro, and maybe I'm not doing it quite right. Notably, I would really like that the expression returned was interpreted as a string literal by other macros (like `concat!`), or make the macro accept more than just string literals, is that possible?

## Examples

You can find how to use the macro in your code in the [network protocol example](examples/network_protocol.rs) (it's as simple as wrapping your strings with the macro).

```rust
slip!("this string will be encrypted!");
```

The unslip utility takes your secret key as parameter, takes the data to process from stdin and outputs the result to stdout.

For example, if you store the following error in `input.txt`...

```
thread 'main' panicked at '$slip:1:WA5mKhwP74N+g8KjAT6hEA==:J1IxgRDKGxAWyM+uwF4y3ZyRKvysUw==$: $slip:1:p6NIauAikdOUN1Iw5OCc9Q==:ZIqdMcLD4q2dsOKaWw==$[3, 1, 2]'
```

...and then run...

```
$ unslip decrypt 15478569587452125874565845212565 < input.txt > output.txt
```

...`output.txt` will contain the following human-friendly message!

```
thread 'main' panicked at 'failed to parse packet: packet data: [3, 1, 2]'
```

## License

Licensed under the MIT license ([license](LICENSE) or http://opensource.org/licenses/MIT).