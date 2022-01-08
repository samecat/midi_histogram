use midly::Smf;

use std::env::args;

use std::fs;

fn main() {
    let first_argument: String = args().nth(1).unwrap();
    let bytes = fs::read(first_argument).unwrap();
    //let bytes = fs::read("../test-asset/C_Major_Chord.mid").unwrap();
    let smf = Smf::parse(&bytes).unwrap();
    
    //initialize 88-bin histogram
    let mut histogram = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let offset : usize = 21;

    for (i, track) in smf.tracks.iter().enumerate() {
        println!("track {} has {} events:", i, track.len());
        for j in track{
            match j.kind{
                midly::TrackEventKind::Midi{channel: _,message} => {
                    match message{
                        midly::MidiMessage::NoteOn{key, vel: _} => {
                            histogram[key.as_int() as usize - offset] += 1;
                        }
                        _ => (),
                    }
                },
                _ => (),
            }
        }
    }

    let mut key : usize = offset;
    for i in histogram{
        println!("{:?}\t:\t{:?}", convert_note_to_pitch(key), i);
        key += 1;
    }
}

fn convert_note_to_pitch(note: usize) -> String {
    match note {
        21 => return "A0".to_string(),
        22 => return "A#0".to_string(),
        23 => return "B0".to_string(),
        24 => return "C1".to_string(),
        25 => return "C#1".to_string(),
        26 => return "D1".to_string(),
        27 => return "D#1".to_string(),
        28 => return "E1".to_string(),
        29 => return "F1".to_string(),
        30 => return "F#1".to_string(),
        31 => return "G1".to_string(),
        32 => return "G#1".to_string(),
        33 => return "A1".to_string(),
        34 => return "A#1".to_string(),
        35 => return "B1".to_string(),
        36 => return "C2".to_string(),
        37 => return "C#2".to_string(),
        38 => return "D2".to_string(),
        39 => return "D#2".to_string(),
        40 => return "E2".to_string(),
        41 => return "F2".to_string(),
        42 => return "F#2".to_string(),
        43 => return "G2".to_string(),
        44 => return "G#2".to_string(),
        45 => return "A2".to_string(),
        46 => return "A#2".to_string(),
        47 => return "B2".to_string(),
        48 => return "C3".to_string(),
        49 => return "C#3".to_string(),
        50 => return "D3".to_string(),
        51 => return "D#3".to_string(),
        52 => return "E3".to_string(),
        53 => return "F3".to_string(),
        54 => return "F#3".to_string(),
        55 => return "G3".to_string(),
        56 => return "G#3".to_string(),
        57 => return "A3".to_string(),
        58 => return "A#3".to_string(),
        59 => return "B3".to_string(),
        60 => return "C4".to_string(),
        61 => return "C#4".to_string(),
        62 => return "D4".to_string(),
        63 => return "D#4".to_string(),
        64 => return "E4".to_string(),
        65 => return "F4".to_string(),
        66 => return "F#4".to_string(),
        67 => return "G4".to_string(),
        68 => return "G#4".to_string(),
        69 => return "A4".to_string(),
        70 => return "A#4".to_string(),
        71 => return "B4".to_string(),
        72 => return "C5".to_string(),
        73 => return "C#5".to_string(),
        74 => return "D5".to_string(),
        75 => return "D#5".to_string(),
        76 => return "E5".to_string(),
        77 => return "F5".to_string(),
        78 => return "F#5".to_string(),
        79 => return "G5".to_string(),
        80 => return "G#5".to_string(),
        81 => return "A5".to_string(),
        82 => return "A#5".to_string(),
        83 => return "B5".to_string(),
        84 => return "C6".to_string(),
        85 => return "C#6".to_string(),
        86 => return "D6".to_string(),
        87 => return "D#6".to_string(),
        88 => return "E6".to_string(),
        89 => return "F6".to_string(),
        90 => return "F#6".to_string(),
        91 => return "G6".to_string(),
        92 => return "G#6".to_string(),
        93 => return "A6".to_string(),
        94 => return "A#6".to_string(),
        95 => return "B6".to_string(),
        96 => return "C7".to_string(),
        97 => return "C#7".to_string(),
        98 => return "D7".to_string(),
        99 => return "D#7".to_string(),
        100 => return "E7".to_string(),
        101 => return "F7".to_string(),
        102 => return "F#7".to_string(),
        103 => return "G7".to_string(),
        104 => return "G#7".to_string(),
        105 => return "A7".to_string(),
        106 => return "A#7".to_string(),
        107 => return "B7".to_string(),
        108 => return "C8".to_string(),
        _ => return "".to_string(),
    }
}