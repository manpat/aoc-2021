use std::time::Instant;

pub struct Timer(Instant);

impl Timer {
	pub fn new() -> Timer {
		Timer(Instant::now())
	}
}

impl Drop for Timer {
	fn drop(&mut self) {
		let duration = self.0.elapsed();
		let seconds = duration.as_secs_f64();

		if seconds < 1.0 / 1000.0 {
			println!("{}us", seconds * 1000.0 * 1000.0);
		} else {
			println!("{}ms", seconds * 1000.0);
		}
	}
}