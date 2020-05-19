#![cfg_attr(
    not(feature = "std"),
    no_std,
    feature(alloc),
    feature(alloc_prelude),
    feature(prelude_import)
)]

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

#[cfg(not(feature = "std"))]
pub(crate) mod prelude {
    pub use alloc::prelude::v1::*;
    pub use core::prelude::v1::*;
}

#[cfg(not(feature = "std"))]
#[allow(unused)]
#[prelude_import]
use crate::prelude::*;

extern crate amcl;
#[cfg(feature = "std")]
#[macro_use]
extern crate lazy_static;
extern crate rand;

mod aggregates;
mod amcl_utils;
mod keys;
mod signature;

use self::amcl::bls381 as BLSCurve;

pub use aggregates::{AggregatePublicKey, AggregateSignature};
pub use amcl_utils::{AmclError, G1_BYTES, G2_BYTES, SECRET_KEY_BYTES};
pub use keys::{Keypair, PublicKey, SecretKey};
pub use signature::Signature;
