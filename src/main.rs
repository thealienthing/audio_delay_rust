
use std::fs::File;
use std::path::Path;

use wav::bit_depth::BitDepth;

const SAMPLE_RATE: u32 = 44100;
const SAMPLES_PER_SEC: u32 = SAMPLE_RATE * 4;

struct Delay {
    buffer: [f32; SAMPLES_PER_SEC as usize],
    delay_val: u32,
    read_index: u32,
    write_index: u32,
    gain: f32
}

impl Delay {
    fn write(&mut self, in_buf: &mut [f32]) {
        for i in 0..in_buf.len() {
            let out_val = self.buffer[self.read_index as usize] * self.gain + in_buf[i];
            in_buf[i] = out_val;
            self.buffer[self.write_index as usize] = out_val;
            self.read_index = (self.read_index + 1) % SAMPLES_PER_SEC;
            self.write_index = (self.write_index + 1) % SAMPLES_PER_SEC;
        }
    }
}

fn main() {
    let mut inp_file = File::open(Path::new("./target/debug/drum_loop.wav")).expect("Failed to open wave file");
    let (header, data) = wav::read(&mut inp_file).expect("Failed to read wav file");
    
    match data {
        BitDepth::Eight(_) => println!("EIGHT"),
        BitDepth::Sixteen(_) => println!("SIXTEEN"),
        BitDepth::TwentyFour(_) => println!("TWENTYFOUR"),
        BitDepth::ThirtyTwoFloat(_) => println!("THIRTYTWOBITFLOAT"),
        BitDepth::Empty => println!("EMPTY")
    }

    let mut in_buf = data.try_into_thirty_two_float().expect("Failed to parse into vector");
    
    let mut delay = Delay {
        buffer: [0.0; SAMPLES_PER_SEC as usize],
        delay_val: 4,
        read_index: 0,
        write_index: 33000,
        gain: 0.4
    };

    delay.write(&mut in_buf);
    let mut out_buf: Vec<f32> = vec!();
    for i in in_buf {
        out_buf.push(i);
    }
    
    let out_buf = wav::bit_depth::BitDepth::ThirtyTwoFloat(out_buf);
    let mut out_file = File::create(Path::new("rust_out.wav")).unwrap();
    wav::write(header, &out_buf, &mut out_file).unwrap();   
}
