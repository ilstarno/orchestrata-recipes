/// Input for a maximum-coverage solver. One unit of capacity is one job.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct CompiledCapacity {
    pub worker_capacities: Vec<usize>,
    pub eligible_workers_by_job: Vec<Vec<usize>>,
}
