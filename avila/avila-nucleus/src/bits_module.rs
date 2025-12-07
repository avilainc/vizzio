//! Operações atômicas de baixo nível em bits e bytes.
//!
//! Este módulo centraliza as implementações de aritmética multiprecisão,
//! algoritmos constant-time e rotinas auxiliares utilizadas em toda a
//! pilha criptográfica Ávila.

#[path = "bits/bitwise.rs"]
pub mod bitwise;
#[path = "bits/constant_time.rs"]
pub mod constant_time;
#[path = "bits/division.rs"]
pub mod division;
#[path = "bits/modular.rs"]
pub mod modular;
#[path = "bits/ntt.rs"]
pub mod ntt;
#[path = "bits/u1024_ops.rs"]
pub mod u1024_ops;
#[path = "bits/u128_ops.rs"]
pub mod u128_ops;
#[path = "bits/u2048_ops.rs"]
pub mod u2048_ops;
#[path = "bits/u256_ops.rs"]
pub mod u256_ops;
#[path = "bits/u4096_ops.rs"]
pub mod u4096_ops;
#[path = "bits/u512_ops.rs"]
pub mod u512_ops;
#[path = "bits/u64_ops.rs"]
pub mod u64_ops;
#[path = "bits/wide_mul.rs"]
pub mod wide_mul;

pub use bitwise::*;
pub use constant_time::*;
pub use division::*;
pub use modular::*;
pub use ntt::*;
pub use u1024_ops::*;
pub use u128_ops::*;
pub use u2048_ops::*;
pub use u256_ops::*;
pub use u4096_ops::*;
pub use u512_ops::*;
pub use u64_ops::*;
pub use wide_mul::*;
