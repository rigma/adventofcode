#[derive(Debug)]
pub struct RaceSimulator {
    duration: u64,
    record_distance: u64,
}

impl RaceSimulator {
    pub fn new(duration: u64, record_distance: u64) -> Self {
        Self {
            duration,
            record_distance,
        }
    }

    pub fn simulate_races(&self) -> Vec<(u64, u64)> {
        (0..=self.duration)
            .map(|hold| (hold, hold * (self.duration - hold)))
            .collect()
    }

    pub fn winning_races(&self) -> usize {
        self.simulate_races()
            .iter()
            .filter(|(_, distance)| *distance > self.record_distance)
            .collect::<Vec<_>>()
            .len()
    }
}
