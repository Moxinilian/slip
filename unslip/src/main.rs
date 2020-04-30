use clap::{App, Arg, SubCommand};

use aes_ctr::stream_cipher::generic_array::GenericArray;
use aes_ctr::stream_cipher::{NewStreamCipher, SyncStreamCipher};
use aes_ctr::Aes128Ctr;

use regex::{Captures, Regex};

use rand::Rng;

use std::io::BufRead;

fn main() {
    let matches = App::new("unslip")
        .version("0.1.0")
        .author("Moxinilian <moxinilian@tutanota.com>")
        .about("Reverts slip debug encryption in stdin")
        .subcommand(SubCommand::with_name("key").about("generates a new random SLIP_KEY"))
        .subcommand(
            SubCommand::with_name("decrypt")
                .about("reverts slip debug encryption in stdin")
                .arg(
                    Arg::with_name("SLIP_KEY")
                        .help("16 bytes hexadecimal key string used at build time")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("decrypt") {
        // Decrypt provided tokens

        if let Some(key_string) = matches.value_of("SLIP_KEY") {
            if let Ok(key) = hex::decode(key_string) {
                if key.len() == 16 {
                    let key = GenericArray::from_slice(&key);

                    // matches $slip:1:(base64):(base64)$
                    // very beautiful regex indeed
                    let reg = Regex::new(
                        r"\$slip:1:((?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?):((?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?)\$"
                    ).unwrap();

                    let stdin = std::io::stdin();
                    let handle = stdin.lock();
                    for line in handle.lines() {
                        if let Ok(line) = line {
                            println!(
                                "{}",
                                reg.replace_all(&line, |caps: &Captures| {
                                    // unslip is very conservative about potential matches,
                                    // if something happens to match the regex but is not a valid token
                                    // it will simply ignore it, as we do not want to
                                    // accidentally degrate the provided message
                                    if let Ok(nonce_data) = base64::decode(&caps[1]) {
                                        if nonce_data.len() == 16 {
                                            if let Ok(mut data) = base64::decode(&caps[2]) {
                                                let nonce = GenericArray::from_slice(&nonce_data);
                                                let mut cipher = Aes128Ctr::new(&key, &nonce);
                                                cipher.apply_keystream(&mut data);
                                                String::from_utf8(data)
                                                    .unwrap_or_else(|_| caps[0].to_owned())
                                            } else {
                                                caps[0].to_owned()
                                            }
                                        } else {
                                            caps[0].to_owned()
                                        }
                                    } else {
                                        caps[0].to_owned()
                                    }
                                })
                            );
                        } else {
                            eprintln!("ERROR: failed to read line from stdin");
                        }
                    }
                } else {
                    eprintln!("ERROR: SLIP_KEY is not 16 bytes long but {}", key.len());
                }
            } else {
                eprintln!(
                    "ERROR: SLIP_KEY is not valid hexadecimal but \"{}\"",
                    key_string
                );
            }
        }
    } else if let Some(_) = matches.subcommand_matches("key") {
        // Generate a new key

        let key: [u8; 16] = rand::thread_rng().gen();
        println!("{}", hex::encode(key));
    }
}
