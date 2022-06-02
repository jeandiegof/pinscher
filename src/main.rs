mod bencher;
use bencher::Bencher;

use csv::Writer;
use std::{thread, time::Duration};

fn sleep(d: u64) {
    let t = 5000;
    for _ in 0..(t / d) {
        thread::sleep(Duration::from_millis(d));
    }
}

fn main() {
    let mut csv_writer = Writer::from_path("output.csv").unwrap();

    for d in 1..5 {
        let cpu_time = Bencher::bench(|| sleep(d));
        csv_writer
            .write_record(&[d.to_string(), cpu_time.as_micros().to_string()])
            .unwrap();

        csv_writer.flush().unwrap();
    }
}
