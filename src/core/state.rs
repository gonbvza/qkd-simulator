use std::collections::HashMap;

use crate::{
    models::node::Node,
    models::{detector::Detector, entangled_pair::NewEntangledPair, links::Link, process::Process},
};

#[derive(Debug, Clone)]
pub struct PairKey {
    pub qubit_nr: i32,
    pub process_id: i32,
}

/// Central in-memory state of the QKD simulation.
///
/// This struct must only be mutated by the EventLoop thread.
#[derive(Debug, Clone)]
pub struct SimulationState {
    /// Active entangled pairs indexed by (process_id, qubit_nr)
    pairs: HashMap<(i32, i32), NewEntangledPair>,

    /// Nodes participating in the simulation
    nodes: HashMap<i32, Node>,

    /// Processes
    processes: HashMap<i32, Process>,

    /// Detectors mapped by node id (or detector id depending on your model)
    detectors: HashMap<i32, Detector>,

    /// Links between nodes (optional but useful if you want fast lookup)
    links: HashMap<i32, Link>,
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
    pub fn get_process_mut(&mut self, process_id: i32) -> Option<&mut Process> {
        self.processes.get_mut(&process_id)
    }

    /// Get proces
    pub fn get_process(&self, process_id: i32) -> Option<&Process> {
        self.processes.get(&process_id)
    }

    /// Get mutable detector
    pub fn get_detector_mut(&mut self, detector_id: i32) -> Option<&mut Detector> {
        self.detectors.get_mut(&detector_id)
    }

    /// Get proces
    pub fn get_detector(&self, detector_id: i32) -> Option<&Detector> {
        self.detectors.get(&detector_id)
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
            .cloned()
            .collect()
    }

    /// Borrow nodes, links and pairs mutably at the same time.
    ///
    /// This keeps fields private while still supporting multi-map event handlers.
    pub fn split_nodes_links_pairs_detector_mut(
        &mut self,
    ) -> (
        &mut HashMap<i32, Node>,
        &mut HashMap<i32, Link>,
        &mut HashMap<(i32, i32), NewEntangledPair>,
        &mut HashMap<i32, Detector>,
    ) {
        (
            &mut self.nodes,
            &mut self.links,
            &mut self.pairs,
            &mut self.detectors,
        )
    }

    /// Borrow detectors, nodes and links mutably at the same time.
    pub fn split_detectors_nodes_links_mut(
        &mut self,
    ) -> (
        &mut HashMap<i32, Detector>,
        &mut HashMap<i32, Node>,
        &mut HashMap<i32, Link>,
    ) {
        (&mut self.detectors, &mut self.nodes, &mut self.links)
    }

    /// Borrow pairs, processes and nodes mutably at the same time.
    pub fn split_pairs_processes_nodes_mut(
        &mut self,
    ) -> (
        &mut HashMap<(i32, i32), NewEntangledPair>,
        &mut HashMap<i32, Process>,
        &mut HashMap<i32, Node>,
    ) {
        (&mut self.pairs, &mut self.processes, &mut self.nodes)
    }

    /// Reset simulation state (useful for restarting QKD runs)
    pub fn clear(&mut self) {
        self.pairs.clear();
        self.nodes.clear();
        self.detectors.clear();
        self.links.clear();
    }
}

impl Default for SimulationState {
    fn default() -> Self {
        Self::new()
    }
}
