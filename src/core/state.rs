use std::collections::HashMap;

use crate::{
    core::process::Process,
    models::{
        detector::Detector, entangled_pair::NewEntangledPair, links::Link, measurement::Measurement,
    },
    models::node::Node,
};

/// Central in-memory state of the QKD simulation.
///
/// IMPORTANT:
/// - This struct MUST only be mutated by the EventLoop thread.
/// - Do NOT share it across threads with Arc/Mutex.
/// - CLI/API threads should only send events, not access this directly.
#[derive(Debug, Clone)]
pub struct SimulationState {
    /// Active entangled pairs indexed by (process_id, qubit_nr)
    pub pairs: HashMap<(i32, i32), NewEntangledPair>,

    /// Nodes participating in the simulation
    pub nodes: HashMap<i32, Node>,

    /// Processes
    pub processes: HashMap<i32, Process>,

    /// Detectors mapped by node id (or detector id depending on your model)
    pub detectors: HashMap<i32, Detector>,

    /// Links between nodes (optional but useful if you want fast lookup)
    pub links: HashMap<i32, Link>,
}

impl SimulationState {
    /// Creates an empty simulation state.
    pub fn new() -> Self {
        Self {
            pairs: HashMap::new(),
            nodes: HashMap::new(),
            processes: HashMap::new(),
            detectors: HashMap::new(),
            links: HashMap::new(),
        }
    }

    /// Insert a new entangled pair into state
    pub fn insert_pair(&mut self, pair: NewEntangledPair) {
        self.pairs.insert((pair.process_id, pair.qubit_nr), pair);
    }

    /// Get mutable reference to a pair
    pub fn get_pair_mut(
        &mut self,
        process_id: i32,
        qubit_nr: i32,
    ) -> Option<&mut NewEntangledPair> {
        self.pairs.get_mut(&(process_id, qubit_nr))
    }

    /// Remove a pair (e.g. after completion or failure)
    pub fn remove_pair(&mut self, process_id: i32, qubit_nr: i32) -> Option<NewEntangledPair> {
        self.pairs.remove(&(process_id, qubit_nr))
    }

    /// Insert or update a node
    pub fn upsert_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }

    /// Insert or update detector
    pub fn upsert_detector(&mut self, detector: Detector) {
        self.detectors.insert(detector.id, detector);
    }

    /// Insert or update link
    pub fn upsert_link(&mut self, link: Link) {
        self.links.insert(link.id, link);
    }

    /// Get proces
    pub fn get_proces_mut(&mut self, process_id: i32) -> Option<&mut Process> {
        self.processes.get_mut(&process_id)
    }

    /// Get proces
    pub fn get_detector_mut(&mut self, detector_id: i32) -> Option<&mut Detector> {
        self.detectors.get_mut(&detector_id)
    }

    pub fn push_process(&mut self, mut process: Process) -> i32 {
        let new_id = self.processes.len() as i32;

        process.id = new_id;
        self.processes.insert(new_id, process);

        new_id
    }

    pub fn get_accepted_measurements(&self, process_id: i32) -> Vec<NewEntangledPair> {
        self.pairs
            .values()
            .filter(|pair| pair.process_id == process_id && pair.accepted)
            .filter_map(|pair| Some(pair.clone()))
            .collect()
    }

    /// Reset simulation state (useful for restarting QKD runs)
    pub fn clear(&mut self) {
        self.pairs.clear();
        self.nodes.clear();
        self.detectors.clear();
        self.links.clear();
    }
}
