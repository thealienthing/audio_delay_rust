
use std::fs::File;
use std::path::Path;

use wav::bit_depth::BitDepth;

const SAMPLE_RATE: u32 = 44100;
const SAMPLES_PER_SEC: u32 = SAMPLE_RATE * 4;

struct Delay {
    buffer: [f32; SAMPLES_PER_SEC as usize],
    read_index: u32,
    write_index: u32,
    gain: f32
}

impl Delay {
    fn new(delay_time: u32, delay_gain: f32) -> Delay {
        Delay {
            buffer: [0.0; SAMPLES_PER_SEC as usize],
            read_index: 0,
            write_index: delay_time,
            gain: delay_gain
        }
    }

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
    let mut input_file: String = String::new();
    let mut delay_time: u32 = 0;
    let mut delay_gain: f32 = 0.0;

    let mut args = std::env::args();
    if args.len() < 4 {
        println!("Must pass wav file, delay time in samples, and delay gain (0.0-0.9)")
    }
    else {
        args.next();
        input_file = args.next().expect("Failed to parse arg");
        delay_time = args.next().expect("Failed to parse arg").parse().expect("failed to parse text");
        delay_gain = args.next().expect("Failed to parse arg").parse().expect("failed to parse text");
        println!("Input file: {}", input_file);
        println!("Delay time: {}", delay_time);
        println!("Delay gain: {}", delay_gain);
    }
    let mut inp_file = File::open(Path::new(&input_file[..])).expect("Failed to open wave file");
    let (header, data) = wav::read(&mut inp_file).expect("Failed to read wav file");
    
    match data {
        BitDepth::Eight(_) => println!("EIGHT"),
        BitDepth::Sixteen(_) => println!("SIXTEEN"),
        BitDepth::TwentyFour(_) => println!("TWENTYFOUR"),
        BitDepth::ThirtyTwoFloat(_) => println!("THIRTYTWOBITFLOAT"),
        BitDepth::Empty => println!("EMPTY")
    }

    let mut in_buf = data.try_into_thirty_two_float().expect("Failed to parse into vector");
    
    let mut delay = Delay::new(delay_time, delay_gain);

    delay.write(&mut in_buf);
    let mut out_buf: Vec<f32> = vec!();
    for i in in_buf {
        out_buf.push(i);
    }
    
    let out_buf = wav::bit_depth::BitDepth::ThirtyTwoFloat(out_buf);
    let mut out_file = File::create(Path::new("rust_out.wav")).unwrap();
    wav::write(header, &out_buf, &mut out_file).unwrap();   
}
