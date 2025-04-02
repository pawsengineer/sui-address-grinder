pub fn get_core_ids(limit: Option<usize>) -> Vec<core_affinity::CoreId> {
    let core_ids = core_affinity::get_core_ids().expect("Failed to fetch core count");
    let limit = limit.unwrap_or(core_ids.len());
    let core_ids = core_ids.into_iter().filter(|id| id.id < limit);
    core_ids.collect()
}
