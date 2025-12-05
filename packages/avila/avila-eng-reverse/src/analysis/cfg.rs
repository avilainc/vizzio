use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlFlowGraph {
    pub entry_point: u64,
    pub basic_blocks: HashMap<u64, BasicBlock>,
    pub edges: Vec<CfgEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicBlock {
    pub address: u64,
    pub size: usize,
    pub instructions: Vec<u64>,
    pub successors: Vec<u64>,
    pub predecessors: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfgEdge {
    pub from: u64,
    pub to: u64,
    pub edge_type: EdgeType,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EdgeType {
    Unconditional,
    ConditionalTrue,
    ConditionalFalse,
    Call,
    Return,
}

impl ControlFlowGraph {
    pub fn new(entry_point: u64) -> Self {
        Self {
            entry_point,
            basic_blocks: HashMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_basic_block(&mut self, block: BasicBlock) {
        self.basic_blocks.insert(block.address, block);
    }

    pub fn add_edge(&mut self, from: u64, to: u64, edge_type: EdgeType) {
        self.edges.push(CfgEdge { from, to, edge_type });

        if let Some(block) = self.basic_blocks.get_mut(&from) {
            if !block.successors.contains(&to) {
                block.successors.push(to);
            }
        }

        if let Some(block) = self.basic_blocks.get_mut(&to) {
            if !block.predecessors.contains(&from) {
                block.predecessors.push(from);
            }
        }
    }

    pub fn get_reachable_blocks(&self, start: u64) -> HashSet<u64> {
        let mut reachable = HashSet::new();
        let mut stack = vec![start];

        while let Some(addr) = stack.pop() {
            if reachable.insert(addr) {
                if let Some(block) = self.basic_blocks.get(&addr) {
                    for &succ in &block.successors {
                        if !reachable.contains(&succ) {
                            stack.push(succ);
                        }
                    }
                }
            }
        }

        reachable
    }

    pub fn find_loops(&self) -> Vec<Vec<u64>> {
        // Implementação básica de detecção de loops
        let mut loops = Vec::new();

        for (&addr, block) in &self.basic_blocks {
            for &succ in &block.successors {
                if succ <= addr {
                    // Possível back edge - indica loop
                    loops.push(vec![addr, succ]);
                }
            }
        }

        loops
    }
}
