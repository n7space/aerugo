//! Module containing Parallel I/O (PIO) pin items for generic I/O pin.

use core::marker::PhantomData;

use super::port_metadata::{IoPortMetadata, RegisterBlock};

/// Structure representing an I/O pin.
///
/// # Generic parameters
/// * `Port` - PAC PIO port type. This type indicated to which PIO port
///            this pin belongs to. For example, for `PC8`, this would be `PIOC`.
/// * `N` - Number of the pin. For example, for `PC8`, this would be `8`.
/// * `PortMode` - Current mode of the pin.
///
/// # Safety
/// Instances of this type should never be constructed manually. Instead, `Port` instance should
/// be used to create it's pin instances. That will guarantee that there will be no
/// duplicate pins, and all the pins will point to correct bits in PIO registers.
///
/// **Make sure to enable PIO clock via PMC driver before using it!**
pub struct Pin<Port: IoPortMetadata, const N: u8, PortMode: Mode> {
    /// PIO port metadata.
    _port_meta: PhantomData<Port>,
    /// Current mode.
    _mode: PhantomData<PortMode>,
    /// Phantom pointer which disables auto-implementation of Send and Sync.
    /// This structure cannot be shared between threads safely, as it uses
    /// raw pointer. Normally, Rust should automatically not implement Send/Sync
    /// for this type because of that, but this pointer is hidden in type system,
    /// under `Port` generic argument, and Rust does not recognize this as something
    /// that should disable auto-implementation of Send and Sync, so we need to do
    /// this manually.
    _disable_send_and_sync: PhantomData<*const ()>,
}

/// Assuming that the user does not create Pin instances manually, it's safe to send them to
/// other threads, as there cannot be duplicate instances of them, sharing data.
/// They are sharing the pointer to PIOx controller's register, but all internal operations
/// are masked. Therefore, if there's no duplicate instances of the pins (as there shouldn't
/// be, by design), each pin refers to different bit in PIOx registers, and there's no memory overlap.
/// Since we're working on single-core environment, parallel access to PIO registers is not possible
/// (as long as DMA is not used, but it's not reasonably possible to implement safety measures for
/// that at this point, so the user should manually manage the safety of DMA operations)
///
/// Sharing references to pins is not safe, and should be managed by the user manually,
/// usually by wrapping pins in type that implements [`Sync`].
unsafe impl<Port: IoPortMetadata, const N: u8, PortMode: Mode> Send for Pin<Port, N, PortMode> {}

/// Trait representing I/O pin's mode.
pub trait Mode {}

/// Empty structure representing I/O pin in post-reset state.
pub struct PostReset;
/// Empty structure representing I/O pin in I/O state (controlled by PIO).
pub struct IO;
/// Empty structure representing I/O pin in peripheral-controlled state.
pub struct Peripheral;

impl Mode for PostReset {}
impl Mode for IO {}
impl Mode for Peripheral {}

/// Enumeration representing available pull-up/down resistors configuration for PIO pin.
pub enum PullResistor {
    /// Neither pull-up nor pull-down is enabled.
    None,
    /// Pull-up is enabled.
    Up,
    /// Pull-down is enabled.
    Down,
}

/// Pin implementation for all possible states.
impl<Port: IoPortMetadata, const ID: u8, PortMode: Mode> Pin<Port, ID, PortMode> {
    /// Returns the number of the pin.
    #[inline(always)]
    pub const fn id(&self) -> u8 {
        ID
    }

    /// Returns ID (uppercase letter) of the port of this pin.
    #[inline(always)]
    pub const fn port_id(&self) -> char {
        Port::ID
    }

    /// Reads and returns current logic state of pin's I/O line.
    pub fn state(&self) -> bool {
        self.is_pin_bit_set(self.registers_ref().pdsr.read().bits())
    }

    /// Returns current pull resistor configuration of the pin.
    pub fn pull_resistor(&self) -> PullResistor {
        match (self.is_pulled_up(), self.is_pulled_down()) {
            (true, true) => panic!("unexpected, invalid state of P{}{} - both pull-up and pull-down resistors are active. Is your silicon OK?", self.port_id(), self.id()),
            (true, false) => PullResistor::Up,
            (false, true) => PullResistor::Down,
            (false, false) => PullResistor::None,
        }
    }

    /// Returns `true` if pin is currently pulled up.
    pub fn is_pulled_up(&self) -> bool {
        self.is_pin_bit_set(self.registers_ref().pusr.read().bits())
    }

    /// Returns `true` if pin is currently pulled down.
    pub fn is_pulled_down(&self) -> bool {
        self.is_pin_bit_set(self.registers_ref().ppdsr.read().bits())
    }

    /// Sets pull resistor configuration of the pin.
    ///
    /// # Parameters
    /// * `resistor` - Desired pull resistor configuration.
    pub fn set_pull_resistor(&mut self, resistor: PullResistor) {
        match resistor {
            PullResistor::None => self.disable_pull_resistor(),
            PullResistor::Up => self.pull_up(),
            PullResistor::Down => self.pull_down(),
        }
    }

    /// Enables pull-up resistor of the pin.
    ///
    /// # Safety
    /// Using this while pin has pull-down resistor enabled will disable the pull-down
    /// before enabling pull-up.
    pub fn pull_up(&mut self) {
        self.disable_pull_resistor();
        // Safety: See `Pin::pin_mask` description.
        self.registers_ref()
            .puer
            .write(|w| unsafe { w.bits(self.pin_mask()) })
    }
    /// Enables pull-down resistor of the pin.
    ///
    /// # Safety
    /// Using this while pin has pull-down resistor enabled will disable the pull-up
    /// before enabling pull-down.
    pub fn pull_down(&mut self) {
        self.disable_pull_resistor();
        // Safety: See `Pin::pin_mask` description.
        self.registers_ref()
            .ppder
            .write(|w| unsafe { w.bits(self.pin_mask()) })
    }

    /// Disables pull-up or pull-down if it's currently enabled.
    /// Does nothing otherwise.
    pub fn disable_pull_resistor(&mut self) {
        self.registers_ref()
            .pudr
            .write(|w| unsafe { w.bits(self.pin_mask()) });
        // Safety: See `Pin::pin_mask` description.
        self.registers_ref()
            .ppddr
            .write(|w| unsafe { w.bits(self.pin_mask()) });
    }

    /// Returns a reference to PIO port registers.
    ///
    /// # Safety
    /// This function dereferences a raw pointer.
    /// It's safe to use as long as there aren't multiple instances of PIO ports sharing the same
    /// PIO instance.
    ///
    /// Note that all PIO pins of single PIO peripheral share the same registers block.
    /// This is due to the hardware implementation of PIO peripheral.
    /// Each PIO pin has it's own bit in every PIO register.
    /// Implementation of this type makes sure that pins always access it's own bit, and don't access
    /// nor modify bits for other pins, therefore sharing this register block should be safe, as long
    /// as all pin types have correct, non-repeating `N` generic parameter values.
    const fn registers_ref(&self) -> &RegisterBlock {
        unsafe { &*Port::REGISTERS }
    }

    /// Returns register mask for current pin.
    ///
    /// # Safety
    /// This function will panic if pin was created with invalid ID, enforcing safe design of this type.
    /// This makes it safe to use with `bits` method of register writer, as it guarantees that returned
    /// mask will always point to a valid bit in 32-bit PIO register.
    const fn pin_mask(&self) -> u32 {
        if ID > 31 {
            panic!("invalid pin number, valid range is (0..=31)")
        }

        1u32 << ID
    }

    /// Helper function that checks whether the bit representing current pin in
    /// provided register's value is set.
    const fn is_pin_bit_set(&self, register_value: u32) -> bool {
        (register_value & self.pin_mask()) != 0
    }

    /// Creates a pin instance.
    /// Does not take arguments, as everything is kept in type system.
    /// This function should never be called manually, only [`Port`](super::Port) should be able
    /// to create pins instances.
    pub(super) const fn new() -> Self {
        Self {
            _port_meta: PhantomData,
            _mode: PhantomData,
            _disable_send_and_sync: PhantomData,
        }
    }
}
