use std::time::Duration;


#[derive(Debug, Clone)]
pub struct FrameInfo {
    pub frame_index: u64,
    pub average_fps: f64,
    pub delta_time: Duration,
    pub runtime: Duration,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn dur_test() {
        let dur = (Duration::from_secs(3) + Duration::from_secs(3) + Duration::from_secs(3)) / 111u32;
        println!("{}", dur.as_micros());
    }
}