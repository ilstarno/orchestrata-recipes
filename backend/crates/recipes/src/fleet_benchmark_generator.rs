use crate::{
    FleetBenchmark, FleetStop, FleetStopKind,
    fleet_benchmark_constants::{
        GRID_METERS, MIN_SERVICE_MINUTES, MIN_WEIGHT_GRAMS, SERVICE_MINUTE_CHOICES, WEIGHT_CHOICES,
    },
};

const MIX_INCREMENT: u64 = 0x9e37_79b9_7f4a_7c15;
const MIX_FACTOR_1: u64 = 0xbf58_476d_1ce4_e5b9;
const MIX_FACTOR_2: u64 = 0x94d0_49bb_1331_11eb;
const SAMPLES_PER_STOP: u64 = 5;
const SHIPMENT_WEIGHT_SAMPLE: u64 = 4;

pub(super) fn generate_stop(benchmark: &FleetBenchmark, id: u32) -> FleetStop {
    let shipment_number = id.div_ceil(2);
    let singleton = id == benchmark.stop_count && id % 2 == 1;
    let (kind, shipment_id, weight_grams) = if singleton {
        (FleetStopKind::Singleton, None, 0)
    } else {
        let kind = if id % 2 == 1 {
            FleetStopKind::Pickup
        } else {
            FleetStopKind::Delivery
        };
        let weight = MIN_WEIGHT_GRAMS
            + sample(
                benchmark.seed,
                u64::from(shipment_number),
                SHIPMENT_WEIGHT_SAMPLE,
                WEIGHT_CHOICES,
            );
        (kind, Some(shipment_number), weight)
    };

    let stop_id = u64::from(id);
    let seed = benchmark.seed;
    let service_minutes = MIN_SERVICE_MINUTES + sample(seed, stop_id, 0, SERVICE_MINUTE_CHOICES);
    let third = benchmark.shift_minutes / 3;
    let window_offset = sample(seed, stop_id, 1, third / 2);
    let (earliest_minute, latest_minute) = match kind {
        FleetStopKind::Pickup => (window_offset, third),
        FleetStopKind::Delivery => {
            let latest = benchmark.shift_minutes - service_minutes;
            ((2 * third + window_offset).min(latest), latest)
        }
        FleetStopKind::Singleton => (window_offset, benchmark.shift_minutes - service_minutes),
    };

    FleetStop {
        id,
        shipment_id,
        kind,
        weight_grams,
        x_meters: sample(seed, stop_id, 2, GRID_METERS),
        y_meters: sample(seed, stop_id, 3, GRID_METERS),
        earliest_minute,
        latest_minute,
        service_minutes,
    }
}

/// Fixed SplitMix64 mixing with per-record sample slots for portable repeatability.
fn sample(seed: u64, id: u64, slot: u64, bound: u32) -> u32 {
    let index = id * SAMPLES_PER_STOP + slot;
    let mut mixed = seed.wrapping_add(index.wrapping_mul(MIX_INCREMENT));
    mixed = (mixed ^ (mixed >> 30)).wrapping_mul(MIX_FACTOR_1);
    mixed = (mixed ^ (mixed >> 27)).wrapping_mul(MIX_FACTOR_2);
    ((mixed ^ (mixed >> 31)) % u64::from(bound)) as u32
}
