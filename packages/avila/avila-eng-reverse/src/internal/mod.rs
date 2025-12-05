// Implementações internas próprias do Deriax
// Substituindo dependências externas por código nativo 100%

pub mod pe_parser;
pub mod elf_parser;
pub mod disassembler;
pub mod pattern_matcher;

pub use pe_parser::PeParser;
pub use elf_parser::ElfParser;
pub use disassembler::X86Disassembler;
pub use pattern_matcher::PatternMatcher;
