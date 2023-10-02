#![no_std]
#![no_main]

extern crate aerugo;
extern crate calldwell;
extern crate cortex_m;
extern crate cortex_m_rt as rt;

use aerugo::hal::user_peripherals::{CPUID, SCB};
use aerugo::{Aerugo, InitApi, SystemHardwareConfig};
use calldwell::write_str;
use rt::entry;

// Align for dcache line size
#[repr(align(32))]
#[derive(Debug)]
struct DummyData {
    // D-cache line size is 32B, data must be multiple of 32B
    // 128B spans 4 d-cache lines.
    pub data: [u8; 128],
}

impl Default for DummyData {
    fn default() -> Self {
        Self { data: [0u8; 128] }
    }
}

fn perform_dcache_tests(scb: &mut SCB, cpuid: &mut CPUID) {
    // Check cache enabling/disabling
    scb.enable_dcache(cpuid);
    assert!(
        SCB::dcache_enabled(),
        "D-Cache was enabled, yet it's reported as disabled"
    );

    // Clearing and invalidating cache has no observable effect outside of processor's core.
    // Exact behavior was verified by analysis.
    let mut dummy = DummyData::default();
    // Safety: This is safe as long as `dummy` is aligned to 32 bytes,
    // and it's data length is a multiple of 32.
    assert!(core::mem::align_of::<DummyData>() % 32 == 0);
    assert!(core::mem::size_of::<DummyData>() % 32 == 0);
    unsafe {
        scb.clean_dcache_by_slice(&dummy.data);
        scb.invalidate_dcache_by_slice(&mut dummy.data);
    }

    scb.clean_invalidate_dcache(cpuid);

    // TODO: Migrate from RTT to UART, because with RTT the cache must be
    // disabled as currently it's not possible to disable RTT region
    // caching due to lack of support from `rtt_target` library for that.
    // scb.disable_dcache(cpuid);
    // assert!(
    //     !SCB::dcache_enabled(),
    //     "D-Cache was disabled, yet it's reported as enabled"
    // );

    write_str("dcache tests successful");
}

fn perform_icache_tests(scb: &mut SCB) {
    // Check cache enabling/disabling
    scb.enable_icache();
    assert!(
        SCB::icache_enabled(),
        "I-Cache was enabled, yet it's reported as disabled"
    );

    // Clearing and invalidating cache has no observable effect outside of processor's core.
    // Exact behavior was verified by analysis.
    scb.invalidate_icache();

    scb.disable_icache();
    assert!(
        !SCB::icache_enabled(),
        "I-Cache was disabled, yet it's reported as enabled"
    );
    write_str("icache tests successful");
}

fn perform_scb_tests(mut scb: SCB, mut cpuid: CPUID) {
    perform_icache_tests(&mut scb);
    perform_dcache_tests(&mut scb, &mut cpuid);
    write_str("all tests finished successfully");
}

#[entry]
fn main() -> ! {
    calldwell::start_session();
    let (aerugo, mut peripherals) = Aerugo::initialize(SystemHardwareConfig::default());

    let mut scb = peripherals.scb.take().unwrap();
    let mut cpu_id = peripherals.cpu_id.take().unwrap();
    scb.disable_dcache(&mut cpu_id);
    scb.disable_icache();

    perform_scb_tests(scb, cpu_id);

    aerugo.start();
}
