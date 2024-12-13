use crate::types::memory::{MemoryType, MemoryStrength};

pub struct WorkingMemory {
    capacity: usize,
    current_load: usize,
    priority_queue: BinaryHeap<MemoryItem>,
}

pub struct LongTermMemory {
    storage: BTreeMap<MemoryId, MemoryNode>,
    indexes: Vec<MemoryIndex>,
    consolidation_scheduler: ConsolidationScheduler,
}