use std::io::Write;

use crate::words::PoopleGraph;

pub struct PoopleSolverApp {
    pub graph: PoopleGraph,
}
const DATABASE: &'static str = include_str!("list.txt");
impl PoopleSolverApp {
    const CACHE_DIR: &'static str = "cache.cbor";
    pub fn new() -> anyhow::Result<Self> {
        let graph = if Self::has_cache() {
            let cache = std::fs::read(Self::CACHE_DIR).unwrap();
            serde_cbor::from_slice(&cache)?
        } else {
            PoopleGraph::new(&DATABASE)
        };
        let this = Self { graph };
        this.create_cache();
        anyhow::Ok(this)
    }
    pub fn find(&self, from: &str, to: &str) -> Option<crate::words::PooplePath> {
        self.graph.find_path(from, to)
    }
    fn has_cache() -> bool {
        std::fs::exists(Self::CACHE_DIR).unwrap_or(false)
    }
    fn create_cache(&self) {
        let mut file = if let Ok(file) = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(Self::CACHE_DIR) {
            file
        } else {
            return;
        };
        let serialized = if let Ok(serialized) = serde_cbor::to_vec(&self.graph) {
            serialized
        } else {
            return;
        };
        let _ = file.write(&serialized);
    }
}