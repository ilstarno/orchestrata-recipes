use crate::FleetStopKind;

/// One synthetic stop. Coordinates are local grid meters, not GPS.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FleetStop {
    pub id: u32,
    pub shipment_id: Option<u32>,
    pub kind: FleetStopKind,
    pub weight_grams: u32,
    pub x_meters: u32,
    pub y_meters: u32,
    pub earliest_minute: u32,
    /// Latest minute to start service, relative to shift start.
    pub latest_minute: u32,
    pub service_minutes: u32,
}
