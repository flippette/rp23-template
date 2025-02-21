#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type, impl_trait_in_bindings)]
#![expect(unstable_features)]

mod error;
mod macros;
mod rt;

use defmt::info;
use embassy_executor::Spawner;
use error::Error;

async fn main(_s: Spawner) -> Result<(), Error> {
    let _p = embassy_rp::init(<_>::default());
    info!("HAL init!");

    Ok(())
}
