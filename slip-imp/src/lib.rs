extern crate proc_macro;
use absolution::{LitKind, Literal, TokenStream, TokenTree};
use proc_macro_error::{abort, proc_macro_error};
use proc_macro_hack::proc_macro_hack;

use aes_ctr::stream_cipher::generic_array::GenericArray;
use aes_ctr::stream_cipher::{NewStreamCipher, SyncStreamCipher};
use aes_ctr::Aes128Ctr;

use rand::Rng;

use quote::quote;

/// Proc macro as expressions are not stable yet, so we need to use a hack like this to achieve it.
#[proc_macro_error]
#[proc_macro_hack]
pub fn slip(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let abs_stream: TokenStream = stream.clone().into();
    if abs_stream.tokens.len() != 0 {
        if let TokenTree::Literal(Literal {
            kind: LitKind::Str(val),
            ..
        }) = &abs_stream.tokens[0]
        {
            if abs_stream.tokens.len() == 1 {
                if let Ok(key_string) = std::env::var("SLIP_KEY") {
                    if let Ok(key) = hex::decode(key_string) {
                        if key.len() == 16 {
                            let key = GenericArray::from_slice(&key);
                            let nonce_data = rand::thread_rng().gen::<[u8; 16]>();
                            let nonce = GenericArray::from_slice(&nonce_data);
                            let mut cipher = Aes128Ctr::new(&key, &nonce);

                            let mut data = val.to_string().into_bytes();
                            cipher.apply_keystream(&mut data);

                            // format: $slip:1:[base64nonce]:[base64data]$
                            // 10 bytes for wrapping, 24 for base64nonce and (4 + data.len() * 4 / 3) bytes for base64data
                            let mut encoded = String::with_capacity(38 + data.len() * 4 / 3); 
                            encoded.push_str("$slip:1:");
                            base64::encode_config_buf(nonce, base64::STANDARD, &mut encoded);
                            encoded.push(':');
                            base64::encode_config_buf(data, base64::STANDARD, &mut encoded);
                            encoded.push('$');

                            quote!({
                                #encoded
                            })
                            .into()
                        } else {
                            abort!(abs_stream, "the provided environment variable SLIP_KEY to encrypt this string is not 16 bytes long")
                        }
                    } else {
                        abort!(abs_stream, "the provided environment variable SLIP_KEY to encrypt this string is not valid hexadecimal")
                    }
                } else {
                    #[cfg(all(not(debug_assertions), not(feature = "allow-no-encryption")))]
                    {
                        proc_macro_error::abort!(abs_stream, "environment variable SLIP_KEY is not set to encrypt this string")
                    }

                    #[cfg(any(debug_assertions, feature = "allow-no-encryption"))]
                    {
                        stream
                    }
                }
            } else {
                abort!(abs_stream, "expected a single string literal")
            }
        } else {
            abort!(abs_stream, "expected a string literal")
        }
    } else {
        abort!(abs_stream, "string literal required for macro")
    }
}
