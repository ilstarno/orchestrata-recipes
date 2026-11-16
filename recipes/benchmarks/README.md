# Synthetic fleet workloads

These are reproducible scale inputs, not delivery histories. The city names label scenarios. The stop counts, courier mix, eight-hour shifts, and seeds are fixed benchmark parameters. Each stop counts one physical visit; a shipment may require more than one stop, so these counts are not parcel or order throughput.

| Scenario | Physical stops | Vehicle / bicycle / walking couriers | Average stops per courier | At least one courier would need | Total shift minutes per stop |
| --- | ---: | ---: | ---: | ---: | ---: |
| [New York](new-york.xml) | 16,030 | 80 / 15 / 5 | 160.30 | 161 stops | 2.994 min |
| [Rome](rome.xml) | 11,221 | 56 / 10 / 4 | 160.30 | 161 stops | 2.994 min |
| [Tirana](tirana.xml) | 1,282 | 6 / 1 / 1 | 160.25 | 161 stops | 2.995 min |

Rome and Tirana are sized to New York's load of 160.30 stops per courier, rounded to whole stops. New York uses the supplied mix and shift. The mixes and 480-minute shifts for Rome and Tirana are assumptions. A seed identifies the repeatable dataset generated from each definition; it does not describe a historical date or route.

## Generate a dataset

From the repository root, print workload statistics or write the stop records to CSV:

```sh
cargo run -p orchestrata-recipes --example fleet_benchmark -- recipes/benchmarks/new-york.xml
cargo run -p orchestrata-recipes --example fleet_benchmark -- recipes/benchmarks/new-york.xml --csv > new-york-stops.csv
```

Use `rome.xml` or `tirana.xml` for the other definitions. Generation pairs pickup and delivery stops, with an explicit unpaired stop when the physical stop count is odd. It does not build routes.

## What the numbers mean

The 161-stop lower bound is the ceiling of stops divided by couriers. It assumes every stop must be assigned once and ignores which courier can serve it. The final column divides total scheduled courier minutes by physical stops. It is an aggregate upper allowance per stop for the entire shift, before deducting breaks or other non-delivery work, not a travel-time or service-time estimate.

There are no observed travel times or actual delivery records here. The definitions show workload scale only; they do not establish route feasibility, an operational deadline, or actual performance. A useful route benchmark must separately specify travel, service duration, eligibility, capacities, time windows and applicable shift rules, then report verified results.
