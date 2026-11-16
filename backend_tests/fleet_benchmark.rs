use orchestrata_recipes::{CourierMix, FleetBenchmarkError, FleetStopKind, load_fleet_benchmark};

const VALID: &str = r#"<fleet-benchmark version="1" id="fixture" city="Synthetic" stops="3" shift-minutes="30" seed="123"><couriers vehicle="2" bicycle="1" walking="0"/></fleet-benchmark>"#;

#[test]
fn fixture_stops_are_repeatable_paired_and_in_bounds_at_all_three_scales() {
    for (stops, vehicle, bicycle, walking) in
        [(16_030, 80, 15, 5), (11_221, 56, 10, 4), (1_282, 6, 1, 1)]
    {
        let xml = format!(
            r#"<fleet-benchmark version="1" id="fixture" city="Synthetic" stops="{stops}" shift-minutes="480" seed="{stops}"><couriers vehicle="{vehicle}" bicycle="{bicycle}" walking="{walking}"/></fleet-benchmark>"#
        );
        let benchmark = load_fleet_benchmark(&xml).unwrap();
        assert_eq!(benchmark.stop_count(), stops);
        assert_eq!(
            benchmark.couriers(),
            CourierMix {
                vehicle,
                bicycle,
                walking
            }
        );

        let rows: Vec<_> = benchmark.stops().collect();
        assert_eq!(rows.len(), stops as usize);
        assert!(benchmark.stops().eq(rows.iter().copied()));

        for (index, row) in rows.iter().enumerate() {
            assert_eq!(row.id, u32::try_from(index).unwrap() + 1);
            assert!(row.x_meters < 20_000 && row.y_meters < 20_000);
            assert!(row.earliest_minute <= row.latest_minute);
            assert!((2..=10).contains(&row.service_minutes));
            assert!(row.latest_minute + row.service_minutes <= benchmark.shift_minutes());
        }

        let (pairs, remainder) = rows.as_chunks::<2>();
        if let [singleton] = remainder {
            assert_eq!(singleton.kind, FleetStopKind::Singleton);
            assert_eq!(singleton.shipment_id, None);
            assert_eq!(singleton.weight_grams, 0);
        }
        assert_eq!(remainder.len(), (stops % 2) as usize);
        for (index, pair) in pairs.iter().enumerate() {
            let pickup = pair[0];
            let delivery = pair[1];
            assert_eq!(pickup.kind, FleetStopKind::Pickup);
            assert_eq!(delivery.kind, FleetStopKind::Delivery);
            assert_eq!(pickup.shipment_id, Some(u32::try_from(index).unwrap() + 1));
            assert_eq!(pickup.shipment_id, delivery.shipment_id);
            assert_eq!(pickup.weight_grams, delivery.weight_grams);
            assert!((100..=10_000).contains(&pickup.weight_grams));
            assert!(pickup.latest_minute + pickup.service_minutes <= delivery.earliest_minute);
        }
    }
}

#[test]
fn minimum_shift_and_singleton_are_supported() {
    let benchmark = load_fleet_benchmark(VALID).unwrap();
    assert_eq!(
        (benchmark.id(), benchmark.city(), benchmark.seed()),
        ("fixture", "Synthetic", 123)
    );
    let rows: Vec<_> = benchmark.stops().collect();
    assert!(rows[0].latest_minute + rows[0].service_minutes <= rows[1].earliest_minute);
    assert!(rows[2].latest_minute + rows[2].service_minutes <= 30);

    let singleton = load_fleet_benchmark(&VALID.replace("stops=\"3\"", "stops=\"1\"")).unwrap();
    assert_eq!(
        singleton.stops().next().unwrap().kind,
        FleetStopKind::Singleton
    );

    let changed = load_fleet_benchmark(&VALID.replace("seed=\"123\"", "seed=\"124\"")).unwrap();
    assert!(!changed.stops().eq(benchmark.stops()));
}

#[test]
fn rejects_unsupported_xml_or_unsafe_structure() {
    let cases = [
        (
            VALID.replace("version=\"1\"", "version=\"2\""),
            FleetBenchmarkError::UnsupportedVersion,
        ),
        (
            VALID.replace("city=", "unknown="),
            FleetBenchmarkError::InvalidStructure,
        ),
        (
            VALID.replace(" city=\"Synthetic\"", ""),
            FleetBenchmarkError::MissingAttribute("city"),
        ),
        (
            VALID.replace("id=\"fixture\"", "id=\"Bad-ID\""),
            FleetBenchmarkError::InvalidIdentifier,
        ),
        (
            VALID.replace("city=\"Synthetic\"", "city=\" \""),
            FleetBenchmarkError::InvalidCity,
        ),
        (
            VALID.replace("<couriers ", "<couriers xmlns=\"urn:test\" "),
            FleetBenchmarkError::InvalidStructure,
        ),
        (
            VALID.replace("/>", "><nested/></couriers>"),
            FleetBenchmarkError::InvalidStructure,
        ),
        (
            VALID.replace("<couriers ", "text<couriers "),
            FleetBenchmarkError::InvalidStructure,
        ),
        (
            format!("<!DOCTYPE fleet-benchmark>{VALID}"),
            FleetBenchmarkError::ForbiddenDeclaration,
        ),
        (
            format!("<?unexpected something?>{VALID}"),
            FleetBenchmarkError::ForbiddenDeclaration,
        ),
        (
            VALID.replace("</fleet-benchmark>", "</other>"),
            FleetBenchmarkError::MalformedXml,
        ),
        (
            format!("{}{}", " ".repeat(4_097), VALID),
            FleetBenchmarkError::TooLarge,
        ),
    ];
    for (xml, expected) in cases {
        assert_eq!(load_fleet_benchmark(&xml), Err(expected), "{xml}");
    }
}

#[test]
fn rejects_out_of_bounds_counts_and_numbers() {
    let cases = [
        (
            "stops=\"3\"",
            "stops=\"0\"",
            FleetBenchmarkError::OutOfRange("stops"),
        ),
        (
            "stops=\"3\"",
            "stops=\"250001\"",
            FleetBenchmarkError::OutOfRange("stops"),
        ),
        (
            "shift-minutes=\"30\"",
            "shift-minutes=\"29\"",
            FleetBenchmarkError::OutOfRange("shift-minutes"),
        ),
        (
            "shift-minutes=\"30\"",
            "shift-minutes=\"1441\"",
            FleetBenchmarkError::OutOfRange("shift-minutes"),
        ),
        (
            "seed=\"123\"",
            "seed=\"-1\"",
            FleetBenchmarkError::InvalidNumber("seed"),
        ),
        (
            "seed=\"123\"",
            "seed=\"18446744073709551616\"",
            FleetBenchmarkError::InvalidNumber("seed"),
        ),
        (
            "vehicle=\"2\"",
            "vehicle=\"10000\"",
            FleetBenchmarkError::OutOfRange("couriers"),
        ),
        (
            "vehicle=\"2\" bicycle=\"1\"",
            "vehicle=\"0\" bicycle=\"0\"",
            FleetBenchmarkError::OutOfRange("couriers"),
        ),
    ];
    for (from, to, expected) in cases {
        assert_eq!(
            load_fleet_benchmark(&VALID.replace(from, to)),
            Err(expected)
        );
    }
}
