// Control Flow Graph builder
use std::collections::{HashMap, HashSet};
use std::error::Error;

/// CFG builder for complete control flow analysis
pub struct CfgBuilder {
    nodes: HashMap<u64, CfgNode>,
    edges: Vec<CfgEdge>,
}

#[derive(Debug, Clone)]
pub struct CfgNode {
    pub address: u64,
    pub instructions: Vec<Instruction>,
    pub node_type: NodeType,
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub address: u64,
    pub mnemonic: String,
    pub operands: Vec<String>,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Entry,
    Exit,
    Basic,
    Branch,
    Call,
    Return,
}

#[derive(Debug, Clone)]
pub struct CfgEdge {
    pub from: u64,
    pub to: u64,
    pub edge_type: EdgeType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EdgeType {
    Fallthrough,
    ConditionalTrue,
    ConditionalFalse,
    Unconditional,
    Call,
    Return,
}

impl CfgBuilder {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    /// Build CFG from binary data
    pub fn build(&mut self, data: &[u8], entry_point: u64) -> Result<(), Box<dyn Error>> {
        let mut worklist = vec![entry_point];
        let mut visited = HashSet::new();

        // Fase 1: Identificar basic blocks começando do entry point
        while let Some(address) = worklist.pop() {
            if visited.contains(&address) {
                continue;
            }
            visited.insert(address);

            // Desassemblar basic block a partir deste endereço
            let block = self.extract_basic_block(data, address)?;

            // Adicionar nó ao CFG
            let node_type = self.determine_node_type(&block);
            let node = CfgNode {
                address,
                instructions: block.clone(),
                node_type,
            };
            self.add_node(node);

            // Analisar última instrução para determinar próximos blocos
            if let Some(last_instr) = block.last() {
                let targets = self.get_branch_targets(last_instr, address);

                for (target, edge_type) in targets {
                    // Adicionar edge
                    self.add_edge(CfgEdge {
                        from: address,
                        to: target,
                        edge_type,
                    });

                    // Adicionar target à worklist se ainda não visitado
                    if !visited.contains(&target) {
                        worklist.push(target);
                    }
                }
            }
        }

        Ok(())
    }

    /// Extrair basic block começando em um endereço
    fn extract_basic_block(&self, data: &[u8], start: u64) -> Result<Vec<Instruction>, Box<dyn Error>> {
        let mut instructions = Vec::new();
        let mut current_addr = start;
        let offset = start as usize;

        if offset >= data.len() {
            return Ok(instructions);
        }

        // Desassemblar até encontrar um branch/call/ret ou atingir limite
        let max_instructions = 100;
        let slice = &data[offset..];

        for i in 0..max_instructions {
            if current_addr as usize >= data.len() {
                break;
            }

            let local_offset = (current_addr - start) as usize;
            if local_offset >= slice.len() {
                break;
            }

            // Decodificar instrução (simplificado)
            let instr = self.decode_simple_instruction(&slice[local_offset..], current_addr)?;
            let is_terminator = self.is_block_terminator(&instr);

            instructions.push(instr.clone());
            current_addr += instr.bytes.len() as u64;

            if is_terminator {
                break;
            }
        }

        Ok(instructions)
    }

    /// Decodificar instrução simples (placeholder - usa lógica básica)
    fn decode_simple_instruction(&self, data: &[u8], address: u64) -> Result<Instruction, Box<dyn Error>> {
        if data.is_empty() {
            return Err("No data to decode".into());
        }

        let opcode = data[0];
        let (mnemonic, length) = match opcode {
            0x90 => ("nop", 1),
            0xC3 => ("ret", 1),
            0xE8 => ("call", 5),
            0xE9 => ("jmp", 5),
            0xEB => ("jmp", 2),
            0x70..=0x7F => ("jcc", 2), // Jump condicional
            0x50..=0x57 => ("push", 1),
            0x58..=0x5F => ("pop", 1),
            _ => ("unknown", 1),
        };

        let bytes = data[..length.min(data.len())].to_vec();

        Ok(Instruction {
            address,
            mnemonic: mnemonic.to_string(),
            operands: vec![],
            bytes,
        })
    }

    /// Verificar se instrução termina um basic block
    fn is_block_terminator(&self, instr: &Instruction) -> bool {
        matches!(instr.mnemonic.as_str(),
            "ret" | "jmp" | "jcc" | "call" | "retn")
    }

    /// Determinar tipo do nó baseado nas instruções
    fn determine_node_type(&self, block: &[Instruction]) -> NodeType {
        if let Some(last) = block.last() {
            match last.mnemonic.as_str() {
                "ret" | "retn" => NodeType::Return,
                "call" => NodeType::Call,
                "jmp" => NodeType::Branch,
                "jcc" => NodeType::Branch,
                _ => NodeType::Basic,
            }
        } else {
            NodeType::Basic
        }
    }

    /// Obter targets de branch de uma instrução
    fn get_branch_targets(&self, instr: &Instruction, block_addr: u64) -> Vec<(u64, EdgeType)> {
        let mut targets = Vec::new();

        match instr.mnemonic.as_str() {
            "call" => {
                // CALL: adicionar target do call e fallthrough
                if let Some(target) = self.extract_call_target(instr) {
                    targets.push((target, EdgeType::Call));
                }
                // Fallthrough após call
                targets.push((instr.address + instr.bytes.len() as u64, EdgeType::Fallthrough));
            }
            "jmp" => {
                // JMP incondicional: apenas target
                if let Some(target) = self.extract_jump_target(instr) {
                    targets.push((target, EdgeType::Unconditional));
                }
            }
            "jcc" => {
                // Jump condicional: target e fallthrough
                if let Some(target) = self.extract_jump_target(instr) {
                    targets.push((target, EdgeType::ConditionalTrue));
                }
                targets.push((instr.address + instr.bytes.len() as u64, EdgeType::ConditionalFalse));
            }
            "ret" | "retn" => {
                // RET: não adiciona targets (fim de função)
            }
            _ => {
                // Instrução normal: fallthrough
                targets.push((instr.address + instr.bytes.len() as u64, EdgeType::Fallthrough));
            }
        }

        targets
    }

    /// Extrair target de um CALL
    fn extract_call_target(&self, instr: &Instruction) -> Option<u64> {
        if instr.bytes.len() >= 5 && instr.bytes[0] == 0xE8 {
            let offset = i32::from_le_bytes([
                instr.bytes[1],
                instr.bytes[2],
                instr.bytes[3],
                instr.bytes[4],
            ]);
            Some((instr.address as i64 + 5 + offset as i64) as u64)
        } else {
            None
        }
    }

    /// Extrair target de um JMP
    fn extract_jump_target(&self, instr: &Instruction) -> Option<u64> {
        if instr.bytes.is_empty() {
            return None;
        }

        match instr.bytes[0] {
            0xE9 if instr.bytes.len() >= 5 => {
                // JMP rel32
                let offset = i32::from_le_bytes([
                    instr.bytes[1],
                    instr.bytes[2],
                    instr.bytes[3],
                    instr.bytes[4],
                ]);
                Some((instr.address as i64 + 5 + offset as i64) as u64)
            }
            0xEB if instr.bytes.len() >= 2 => {
                // JMP rel8
                let offset = instr.bytes[1] as i8;
                Some((instr.address as i64 + 2 + offset as i64) as u64)
            }
            0x70..=0x7F if instr.bytes.len() >= 2 => {
                // Jcc rel8
                let offset = instr.bytes[1] as i8;
                Some((instr.address as i64 + 2 + offset as i64) as u64)
            }
            _ => None,
        }
    }

    /// Add node to CFG
    pub fn add_node(&mut self, node: CfgNode) {
        self.nodes.insert(node.address, node);
    }

    /// Add edge to CFG
    pub fn add_edge(&mut self, edge: CfgEdge) {
        self.edges.push(edge);
    }

    /// Get number of nodes
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Get number of edges
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Get node at address
    pub fn get_node(&self, address: u64) -> Option<&CfgNode> {
        self.nodes.get(&address)
    }

    /// Detect dead code (código não alcançável)
    pub fn find_dead_code(&self) -> Vec<u64> {
        let mut reachable = HashSet::new();
        let mut worklist = Vec::new();

        // Encontrar entry nodes
        for (addr, node) in &self.nodes {
            if node.node_type == NodeType::Entry || *addr == self.get_entry_point() {
                worklist.push(*addr);
            }
        }

        // DFS para marcar nós alcançáveis
        while let Some(addr) = worklist.pop() {
            if reachable.contains(&addr) {
                continue;
            }
            reachable.insert(addr);

            // Adicionar sucessores
            for edge in &self.edges {
                if edge.from == addr && !reachable.contains(&edge.to) {
                    worklist.push(edge.to);
                }
            }
        }

        // Retornar nós não alcançáveis
        self.nodes.keys()
            .filter(|addr| !reachable.contains(addr))
            .copied()
            .collect()
    }

    /// Detect loops (usando algoritmo simples de back-edges)
    pub fn find_loops(&self) -> Vec<Loop> {
        let mut loops = Vec::new();
        let mut visited = HashSet::new();
        let mut in_stack = HashSet::new();

        // DFS para encontrar back-edges
        for addr in self.nodes.keys() {
            if !visited.contains(addr) {
                self.dfs_find_loops(*addr, &mut visited, &mut in_stack, &mut loops);
            }
        }

        loops
    }

    fn dfs_find_loops(
        &self,
        node: u64,
        visited: &mut HashSet<u64>,
        in_stack: &mut HashSet<u64>,
        loops: &mut Vec<Loop>
    ) {
        visited.insert(node);
        in_stack.insert(node);

        // Visitar sucessores
        for edge in &self.edges {
            if edge.from == node {
                if !visited.contains(&edge.to) {
                    self.dfs_find_loops(edge.to, visited, in_stack, loops);
                } else if in_stack.contains(&edge.to) {
                    // Back-edge encontrado - indica loop
                    loops.push(Loop {
                        header: edge.to,
                        body: vec![node],
                        exit_nodes: vec![],
                    });
                }
            }
        }

        in_stack.remove(&node);
    }

    fn get_entry_point(&self) -> u64 {
        // Retornar primeiro nó ou 0
        self.nodes.keys().next().copied().unwrap_or(0)
    }

    /// Detect obfuscation patterns
    pub fn detect_obfuscation(&self) -> ObfuscationReport {
        let mut report = ObfuscationReport {
            has_control_flow_flattening: false,
            has_opaque_predicates: false,
            has_junk_code: false,
            complexity_score: 0.0,
        };

        // Calcular complexidade ciclomática
        let num_nodes = self.nodes.len();
        let num_edges = self.edges.len();
        let num_components = 1; // Simplificado
        report.complexity_score = (num_edges - num_nodes + 2 * num_components) as f64;

        // Detectar control flow flattening
        // (muitos edges de um único dispatcher node)
        for (addr, node) in &self.nodes {
            let outgoing = self.edges.iter()
                .filter(|e| e.from == *addr)
                .count();

            if outgoing > 10 {
                report.has_control_flow_flattening = true;
            }
        }

        // Detectar junk code (basic blocks muito pequenos com muitos jumps)
        let avg_block_size = self.nodes.values()
            .map(|n| n.instructions.len())
            .sum::<usize>() as f64 / num_nodes as f64;

        if avg_block_size < 2.0 {
            report.has_junk_code = true;
        }

        report
    }

    /// Export to DOT format (Graphviz)
    pub fn export_dot(&self) -> String {
        let mut dot = String::from("digraph CFG {\n");
        dot.push_str("  rankdir=TB;\n");
        dot.push_str("  node [shape=box];\n\n");

        // Adicionar nós
        for (addr, node) in &self.nodes {
            let color = match node.node_type {
                NodeType::Entry => "green",
                NodeType::Exit | NodeType::Return => "red",
                NodeType::Branch => "yellow",
                NodeType::Call => "blue",
                _ => "white",
            };

            let label = format!("0x{:x}\\n{} instrs", addr, node.instructions.len());
            dot.push_str(&format!(
                "  node_{:x} [label=\"{}\", fillcolor={}, style=filled];\n",
                addr, label, color
            ));
        }

        dot.push_str("\n");

        // Adicionar edges
        for edge in &self.edges {
            let style = match edge.edge_type {
                EdgeType::ConditionalTrue => "color=green",
                EdgeType::ConditionalFalse => "color=red",
                EdgeType::Call => "color=blue, style=dashed",
                EdgeType::Return => "color=purple, style=dashed",
                _ => "color=black",
            };

            dot.push_str(&format!(
                "  node_{:x} -> node_{:x} [{}];\n",
                edge.from, edge.to, style
            ));
        }

        dot.push_str("}\n");
        dot
    }
}

#[derive(Debug, Clone)]
pub struct Loop {
    pub header: u64,
    pub body: Vec<u64>,
    pub exit_nodes: Vec<u64>,
}

#[derive(Debug, Clone)]
pub struct ObfuscationReport {
    pub has_control_flow_flattening: bool,
    pub has_opaque_predicates: bool,
    pub has_junk_code: bool,
    pub complexity_score: f64,
}
