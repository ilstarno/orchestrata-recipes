use crate::{CourierMix, FleetStop, fleet_benchmark_generator::generate_stop};

/// Validated metadata for a synthetic pickup-and-delivery fixture.
///
/// Courier counts describe the scenario, not a claim that it can be routed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FleetBenchmark {
    pub(crate) id: String,
    pub(crate) city: String,
    pub(crate) stop_count: u32,
    pub(crate) shift_minutes: u32,
    pub(crate) seed: u64,
    pub(crate) couriers: CourierMix,
}

impl FleetBenchmark {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn city(&self) -> &str {
        &self.city
    }

    pub fn stop_count(&self) -> u32 {
        self.stop_count
    }

    pub fn shift_minutes(&self) -> u32 {
        self.shift_minutes
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn couriers(&self) -> CourierMix {
        self.couriers
    }

    /// Generates stops in record order without allocating the whole fixture.
    pub fn stops(&self) -> impl Iterator<Item = FleetStop> + '_ {
        (1..=self.stop_count).map(|id| generate_stop(self, id))
    }
}
