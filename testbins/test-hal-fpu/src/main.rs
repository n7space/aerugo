#![no_std]
#![no_main]

/*
SRS list:
- Support for default NaN operation mode
- Support for full-compliance IEEE operation mode
- Support for flush-to-zero operation mode
- Support for handling invalid operands in default NaN/flush-to-zero mode
- Lazy stacking enabled by default (LSPEN)
*/

extern crate aerugo;
extern crate calldwell;
extern crate cortex_m;
extern crate cortex_m_rt as rt;

use aerugo::{
    hal::drivers::fpu::{
        Config, ContextConfig, ContextStateFlags, FlushToZeroMode, Fpu, HalfPrecisionMode, NaNMode,
        RoundingMode,
    },
    time::MillisDurationU32 as WatchdogDuration,
    Aerugo, InitApi, SystemHardwareConfig,
};
use calldwell::write_str;
use rt::entry;

#[entry]
fn main() -> ! {
    calldwell::start_session();
    let (aerugo, mut peripherals) = Aerugo::initialize(SystemHardwareConfig {
        watchdog_timeout: WatchdogDuration::secs(16),
    });

    let mut scb = peripherals.scb.take().unwrap();
    let mut fpu = Fpu::new(peripherals.fpu.take().unwrap());

    write_str("Performing FPU tests...");

    write_str("Enabling FPU...");
    fpu.enable(&mut scb);
    write_str("FPU enabled!");

    if fpu.is_lazy_stacking_enabled() {
        write_str("Lazy stacking is enabled by default!");
    } else {
        panic!("Lazy stacking is NOT enabled by default!")
    }

    let original_config = fpu.get_config();

    // This config is IEEE-compliant and validates whether the FPU can be configured as such
    let fpu_config_a = Config {
        is_context_preserved_on_exception: false,
        default_context_config: ContextConfig {
            half_precision_mode: HalfPrecisionMode::IEEE754_2008,
            nan_mode: NaNMode::PropagateNaNOperands,
            flush_to_zero_mode: FlushToZeroMode::Disabled,
            ..original_config.default_context_config
        },
        ..original_config
    };

    let fpu_config_b = Config {
        is_context_preserved_on_exception: true,
        default_context_config: ContextConfig {
            half_precision_mode: HalfPrecisionMode::Alternative,
            nan_mode: NaNMode::ReturnDefaultNaN,
            flush_to_zero_mode: FlushToZeroMode::Enabled,
            ..original_config.default_context_config
        },
        ..original_config
    };

    fpu.set_config(fpu_config_a);
    let read_config = fpu.get_config();
    assert_eq!(read_config, fpu_config_a);

    fpu.set_config(fpu_config_b);
    let read_config = fpu.get_config();
    assert_eq!(read_config, fpu_config_b);

    fpu.set_config(original_config);
    let read_config = fpu.get_config();
    assert_eq!(read_config, original_config);

    write_str("Default context configuration test successful!");

    let default_context_config = fpu.context_config();
    let context_cfg_a = ContextConfig {
        half_precision_mode: HalfPrecisionMode::IEEE754_2008,
        nan_mode: NaNMode::PropagateNaNOperands,
        flush_to_zero_mode: FlushToZeroMode::Disabled,
        rounding_mode: RoundingMode::RoundToNearest,
    };

    let context_cfg_b = ContextConfig {
        half_precision_mode: HalfPrecisionMode::Alternative,
        nan_mode: NaNMode::ReturnDefaultNaN,
        flush_to_zero_mode: FlushToZeroMode::Enabled,
        rounding_mode: RoundingMode::RoundTowardsZero,
    };

    fpu.set_context_config(context_cfg_a);
    let read_context_config = fpu.context_config();
    assert_eq!(context_cfg_a, read_context_config);

    fpu.set_context_config(context_cfg_b);
    let read_context_config = fpu.context_config();
    assert_eq!(context_cfg_b, read_context_config);

    fpu.set_context_config(default_context_config);
    let read_context_config = fpu.context_config();
    assert_eq!(default_context_config, read_context_config);

    write_str("FPU context configuration test successful!");

    // FPU state reading is implemented via `cortex_m` and wrapped in FPU driver, therefore
    // we're using pre-existing and tested implementation. This check will validate that the code
    // using that implementation compiles.
    let current_state = fpu.context_state();
    // Since we haven't used FPU, all state flags should be set to false.
    let expected_state_flags = ContextStateFlags {
        negative_condition: false,
        zero_condition: false,
        carry_condition: false,
        overflow_condition: false,
        input_denormal_exception: false,
        inexact_exception: false,
        underflow_exception: false,
        overflow_exception: false,
        division_by_zero_exception: false,
        invalid_operation_exception: false,
    };
    assert_eq!(current_state, expected_state_flags);
    write_str("FPU state checking test successful!");

    write_str("All FPU tests done!");

    aerugo.start();
}
