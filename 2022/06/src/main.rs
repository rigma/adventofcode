use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{self, Read},
};

enum Marker {
    StartOfPacket,
    StartOfMessage,
}

impl Marker {
    const fn size(&self) -> usize {
        match *self {
            Self::StartOfPacket => 4,
            Self::StartOfMessage => 14,
        }
    }
}

fn main() -> io::Result<()> {
    let input = {
        let mut input = String::new();
        let mut file = File::open(env::args().nth(1).expect("input text file expected"))?;
        file.read_to_string(&mut input)?;

        input
    };

    println!("Message length: {}", input.len());

    let (pos, start_marker) = detect_marker(Marker::StartOfPacket, &input);
    println!(
        "Start-of-packet marker: {} ({} characters read)",
        start_marker,
        pos + start_marker.len(),
    );

    let (pos, message_marker) = detect_marker(Marker::StartOfMessage, &input);
    println!(
        "Start-of-message marker: {} ({} characters read)",
        message_marker,
        pos + message_marker.len(),
    );

    Ok(())
}

fn detect_marker(ty: Marker, input: &str) -> (usize, String) {
    let mut offset = 0;

    loop {
        let marker = String::from(&input[offset..offset + ty.size()]);

        let mut character_set = HashSet::new();
        marker.chars().for_each(|char| {
            character_set.insert(char);
        });

        if character_set.len() == marker.chars().count() {
            return (offset, marker);
        }

        offset += 1;
    }
}
