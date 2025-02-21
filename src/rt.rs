//! Runtime setup and background tasks.

use defmt::{info, unwrap};
use defmt_rtt as _;
use embassy_executor::{Spawner, main};
use embassy_rp::block::ImageDef;
use panic_probe as _;

#[used]
#[unsafe(link_section = ".start_block")]
static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

#[main]
async fn _start(s: Spawner) {
    unwrap!(crate::main(s).await);
    info!("main exited!");
}
