use slip::slip;
use anyhow::{Result, Context, anyhow};

// Hiding how packet serialization works is a common usage for
// debug message obfuscation. Here, we use the anyhow crate to
// handle errors.

// You can look at this binary in your favorite assembly analysis tools,
// while the fact that slip is here is apparent, taking advantage of
// debug messages to understand the code is significantly harder.

// In a real life setting, you would send parsing failures to a remote
// error handling store, in which the unslip binary would automatically
// restore encrypted errors into human-readable forms.

pub fn main() {
    // You can try passing the entire output of this panic to unslip to see how
    // it can transparently restore the original error messages.
    // Do not forget the SLIP_KEY environment variable!
    deserialize(vec![3, 1, 2]).expect(slip!("failed to parse packet"));
}

pub fn deserialize(data: Vec<u8>) -> Result<Message> {
    let packet_id = data.get(0).context(slip!("received empty packet"))?;

    let res = match packet_id {
        0 => deserialize_auth(&data[1..]).context(slip!("could not parse auth packet")),
        1 => deserialize_request(&data[1..]).context(slip!("could not parse request packet")),
        _ => Err(anyhow!(slip!("received invalid message type")))
    };

    // Provide the received packet for context.
    // Unfortunately, I cannot figure out how to make procedural
    // macros return actual string literals on stable yet.
    // That means that we need to do runtime concatenation here, but hopefully
    // in the time being this should only happen in cold paths.
    res.with_context(|| [slip!("packet data: "), &format!("{:?}", data)].concat())
}

pub enum Message {
    Authentication { token: u64 },
    Resource { id: u64 },
}

fn deserialize_auth(_data: &[u8]) -> Option<Message> {
    None // Let's pretend it failed.
}

fn deserialize_request(_data: &[u8]) -> Option<Message> {
    None // Let's pretend it failed.
}