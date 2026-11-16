# Free planning examples

Try an example before connecting your data. Pick a scenario from the table, run the sample problem, and compare the result with its checked-in expected plan. Everything here uses fictional data. The examples are included with the checkout; none needs a paid connector or add-on.

| Use case | XML recipe | What the sample actually solves |
| --- | --- | --- |
| Airport operations | [Airport gate assignment](airport-gate-assignment.xml) | Assign three overlapping flights to compatible free gates. |
| Airline operations | [Airline crew assignment](airline-crew-assignment.xml) | Assign three simultaneous duties to distinct, prequalified crews. |
| Call centers | [Call-center staffing](call-center-staffing.xml) | Cover three queues in the same interval using skill and language eligibility. |
| Distribution | [Loading dock assignment](loading-dock-assignment.xml) | Place overlapping trailers at compatible loading doors. |
| Cluster operations | [Cluster workload placement](cluster-workload-placement.xml) | Give three concurrent jobs distinct hosts that meet their precomputed requirements. |
| Hospital operations | [Nonclinical shift coverage](hospital-nonclinical-shift-coverage.xml) | Cover concurrent visitor desk, supply desk and courier intake posts. |
| Dental clinics | [Appointment room allocation](dental-clinic-room-allocation.xml) | Give overlapping appointments separate compatible rooms. |
| High schools | [One-period timetabling](high-school-timetabling.xml) | Assign concurrent classes to rooms with the required facilities. |
| Surgical operations | [Operating room block allocation](operating-room-block-allocation.xml) | Assign preapproved operating-room blocks to distinct compatible rooms. Logistics only. |
| Pickup and delivery | [Fleet assignment](fleet-pickup-delivery.xml) | Use vehicles that can carry one job's weight and cover its entire time window. |
| Healthcare delivery | [Supplies delivery](healthcare-supply-delivery.xml) | Assign vans to overlapping supply jobs with weight and availability limits. |
| Field service | [Technician assignment](field-service.xml) | Cover overlapping service jobs with distinct eligible technicians. |
| Retail workforce | [Two-store staffing what-if](retail-staffing-what-if.xml) | Compare a baseline with a callout across two stores, while retaining a fixed shift. [Run both cases](examples/retail-staffing-what-if/README.md). |

## Run one

With the Orchestrata CLI on your path, from this repository root:

```sh
orchestrata-cli recipes/examples/fleet-pickup-delivery/problem.json
```

The fleet example assigns the parcel pickup to the short-shift van, the depot delivery to the late-start van, and the small-box drop to the large van. The CLI reports `[1,2,0]`, zero hard and soft penalties, and eight evaluations. The [source inputs](examples/fleet-pickup-delivery/inputs.json), [solver problem](examples/fleet-pickup-delivery/problem.json), and [expected result](examples/fleet-pickup-delivery/expected.json) are available to inspect.

With the local API running on its normal address:

```sh
curl -fsS http://127.0.0.1:3000/api/v1/recipes
curl -fsS http://127.0.0.1:3000/api/v1/recipes/fleet-pickup-delivery
curl -fsS http://127.0.0.1:3000/api/v1/recipes/fleet-pickup-delivery/example \
  | curl -fsS -X POST http://127.0.0.1:3000/api/v1/solve \
      -H 'Content-Type: application/json' --data-binary @-
```

## Larger synthetic workloads

The [fleet benchmark cases](benchmarks/README.md) generate repeatable city-scale stop data; they do not solve routes.

## What these examples do not cover

The XML describes the scenario and its fields. The runnable JSON is the normalized problem consumed by the exact solver; the metadata endpoint does not execute XML. These cases use fixed times, exclusive concurrent assignments, and precomputed eligibility. Fleet eligibility is checked against the supplied windows and per-job capacity. Multi-stop routing, cumulative loads, pickup-before-delivery sequencing, full workforce rules, and full school timetables are outside these examples. The healthcare examples contain no patient data and do not make clinical decisions.
