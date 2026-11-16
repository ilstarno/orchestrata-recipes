use std::{
    env,
    error::Error,
    fs::File,
    io::{self, BufWriter, Read, Write},
    path::Path,
};

use orchestrata_recipes::{FleetBenchmark, FleetStopKind, load_fleet_benchmark};

const INPUT_PREFIX_BYTES: u64 = 8_192;
const USAGE: &str = "usage: fleet_benchmark <descriptor.xml> [--csv]";

fn main() -> Result<(), Box<dyn Error>> {
    let mut arguments = env::args_os().skip(1);
    let path = arguments.next().ok_or(USAGE)?;
    let csv = match arguments.next().as_deref() {
        None => false,
        Some(flag) if flag == "--csv" && arguments.next().is_none() => true,
        _ => return Err(USAGE.into()),
    };

    let mut source = String::new();
    File::open(Path::new(&path))?
        .take(INPUT_PREFIX_BYTES)
        .read_to_string(&mut source)?;
    let benchmark = load_fleet_benchmark(&source)
        .map_err(|error| io::Error::other(format!("invalid benchmark descriptor: {error:?}")))?;

    let output = io::stdout();
    let mut writer = BufWriter::new(output.lock());
    if csv {
        write_csv(&benchmark, &mut writer)?;
    } else {
        write_stats(&benchmark, &mut writer)?;
    }
    writer.flush()?;
    Ok(())
}

fn write_stats(benchmark: &FleetBenchmark, output: &mut impl Write) -> io::Result<()> {
    let mut pickups = 0;
    let mut deliveries = 0;
    let mut singletons = 0;
    for stop in benchmark.stops() {
        match stop.kind {
            FleetStopKind::Pickup => pickups += 1,
            FleetStopKind::Delivery => deliveries += 1,
            FleetStopKind::Singleton => singletons += 1,
        }
    }
    let couriers = benchmark.couriers();
    writeln!(
        output,
        "{} ({}): {} stops, {pickups} pickups, {deliveries} deliveries, {singletons} singletons; {} vehicle / {} bicycle / {} walking couriers; {}-minute shift; seed {}",
        benchmark.city(),
        benchmark.id(),
        benchmark.stop_count(),
        couriers.vehicle,
        couriers.bicycle,
        couriers.walking,
        benchmark.shift_minutes(),
        benchmark.seed()
    )?;
    writeln!(
        output,
        "Synthetic fixture only; no routes or feasibility result."
    )
}

fn write_csv(benchmark: &FleetBenchmark, output: &mut impl Write) -> io::Result<()> {
    writeln!(
        output,
        "id,shipment_id,kind,weight_grams,x_meters,y_meters,earliest_minute,latest_minute,service_minutes"
    )?;
    for stop in benchmark.stops() {
        let kind = match stop.kind {
            FleetStopKind::Pickup => "pickup",
            FleetStopKind::Delivery => "delivery",
            FleetStopKind::Singleton => "singleton",
        };
        write!(output, "{},", stop.id)?;
        if let Some(shipment_id) = stop.shipment_id {
            write!(output, "{shipment_id}")?;
        }
        writeln!(
            output,
            ",{kind},{},{},{},{},{},{}",
            stop.weight_grams,
            stop.x_meters,
            stop.y_meters,
            stop.earliest_minute,
            stop.latest_minute,
            stop.service_minutes,
        )?;
    }
    Ok(())
}
