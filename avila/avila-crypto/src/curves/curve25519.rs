//! Curve25519 - Curva moderna para Ed25519
//!
//! Montgomery form: By² = x³ + Ax² + x
//! Twisted Edwards: -x² + y² = 1 + dx²y²
//!
//! Características:
//! - Prime: p = 2²⁵⁵ - 19 (muito eficiente)
//! - Complete formulas (sem casos especiais)
//! - Constant-time por design
//! - Twist secure

use super::{EllipticCurve, Point};
use avila_math::inverse::mod_inverse;
use avila_math::modular::{add_mod, mul_mod, pow_mod, sub_mod};
use avila_primitives::U256;

/// Exponent used for square-root extraction when p ≡ 5 mod 8
/// ((p + 3) / 8) for p = 2^255 - 19.
const SQRT_EXPONENT: U256 = U256 {
    limbs: [
        0xFFFFFFFFFFFFFFFE,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0x0FFFFFFFFFFFFFFF,
    ],
};

/// Curve25519 / Ed25519
pub struct Curve25519;

impl Curve25519 {
    /// p = 2²⁵⁵ - 19
    pub const P_LIMBS: [u64; 4] = [
        0xFFFFFFFFFFFFFFED,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0x7FFFFFFFFFFFFFFF,
    ];

    /// Ordem do subgrupo primo
    pub const L_LIMBS: [u64; 4] = [
        0x5812631A5CF5D3ED,
        0x14DEF9DEA2F79CD6,
        0x0000000000000000,
        0x1000000000000000,
    ];

    /// d = -121665/121666 na forma Edwards
    pub const D_LIMBS: [u64; 4] = [
        0xEB4DCA135978A3,
        0xA4D4141D8AB75EB4,
        0x1806AD2FE478C4EE,
        0x52036CEE2B6FFE73,
    ];

    /// Ponto base em Edwards form
    pub const GX_LIMBS: [u64; 4] = [
        0x8F25D51A657C0710,
        0xC6CB47F5D7789C85,
        0x17EDD3EF5A1A9847,
        0x216936D3CD6E53FE,
    ];

    pub const GY_LIMBS: [u64; 4] = [
        0x6666666666666658,
        0x6666666666666666,
        0x6666666666666666,
        0x6666666666666666,
    ];

    #[inline]
    const fn d() -> U256 {
        U256 {
            limbs: Self::D_LIMBS,
        }
    }

    #[inline]
    pub const fn identity() -> Point {
        Point {
            x: U256::ZERO,
            y: U256::ONE,
        }
    }

    #[inline]
    pub fn negate(point: &Point) -> Point {
        if point.is_infinity() {
            return *point;
        }

        let p = &Self::P;
        let neg_x = sub_mod(p, &point.x, p);

        Point {
            x: neg_x,
            y: point.y,
        }
    }
}

impl EllipticCurve for Curve25519 {
    const NAME: &'static str = "Curve25519 / Ed25519";

    const P: U256 = U256 {
        limbs: Self::P_LIMBS,
    };

    const N: U256 = U256 {
        limbs: Self::L_LIMBS,
    };

    const H: u8 = 8; // Cofator = 8

    const G: Point = Point {
        x: U256 {
            limbs: Self::GX_LIMBS,
        },
        y: U256 {
            limbs: Self::GY_LIMBS,
        },
    };

    fn is_on_curve(point: &Point) -> bool {
        if point.is_infinity() {
            return true;
        }

        let p = &Self::P;
        let d = Self::d();

        let x2 = mul_mod(&point.x, &point.x, p);
        let y2 = mul_mod(&point.y, &point.y, p);
        let lhs = sub_mod(&y2, &x2, p);

        let x2y2 = mul_mod(&x2, &y2, p);
        let rhs = add_mod(&U256::ONE, &mul_mod(&d, &x2y2, p), p);

        lhs == rhs
    }

    fn add(p: &Point, q: &Point) -> Point {
        if p.is_infinity() {
            return *q;
        }
        if q.is_infinity() {
            return *p;
        }

        let modulus = &Self::P;
        let d = Self::d();

        let x1x2 = mul_mod(&p.x, &q.x, modulus);
        let y1y2 = mul_mod(&p.y, &q.y, modulus);
        let x1y2 = mul_mod(&p.x, &q.y, modulus);
        let y1x2 = mul_mod(&p.y, &q.x, modulus);

        let numerator_x = add_mod(&x1y2, &y1x2, modulus);
        let numerator_y = add_mod(&y1y2, &x1x2, modulus);

        let dx1x2y1y2 = mul_mod(&d, &mul_mod(&x1x2, &y1y2, modulus), modulus);
        let denom_x = add_mod(&U256::ONE, &dx1x2y1y2, modulus);
        let denom_y = sub_mod(&U256::ONE, &dx1x2y1y2, modulus);

        let denom_x_inv = match mod_inverse(&denom_x, modulus) {
            Some(inv) => inv,
            None => return Point::INFINITY,
        };

        let denom_y_inv = match mod_inverse(&denom_y, modulus) {
            Some(inv) => inv,
            None => return Point::INFINITY,
        };

        let x3 = mul_mod(&numerator_x, &denom_x_inv, modulus);
        let y3 = mul_mod(&numerator_y, &denom_y_inv, modulus);

        Point { x: x3, y: y3 }
    }

    fn double(p: &Point) -> Point {
        Self::add(p, p)
    }

    fn scalar_mul(k: &U256, p: &Point) -> Point {
        if p.is_infinity() {
            return Point::INFINITY;
        }

        let mut r0 = Point::INFINITY;
        let mut r1 = *p;

        for bit in (0..256).rev() {
            let limb = (bit / 64) as usize;
            let offset = bit % 64;
            let bit_set = (k.limbs[limb] >> offset) & 1;

            if bit_set == 0 {
                r1 = Self::add(&r0, &r1);
                r0 = Self::double(&r0);
            } else {
                r0 = Self::add(&r0, &r1);
                r1 = Self::double(&r1);
            }
        }

        r0
    }
}
