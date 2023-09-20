//! Module containing Parallel I/O (PIO) pin items for generic I/O pin.

pub use super::peripheral_pin::Peripheral;

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
pub struct PostResetMode;
/// Empty structure representing I/O pin in I/O state (controlled by PIO).
pub struct IOMode;
/// Empty structure representing I/O pin in peripheral-controlled state.
pub struct PeripheralMode;

impl Mode for PostResetMode {}
impl Mode for IOMode {}
impl Mode for PeripheralMode {}

/// Enumeration representing available pull-up/down resistors configuration for PIO pin.
pub enum PullResistor {
    /// Neither pull-up nor pull-down is enabled.
    None,
    /// Pull-up is enabled.
    Up,
    /// Pull-down is enabled.
    Down,
}

/// Generic pin functions, available to all pins, no matter which state they are currently in.
impl<Port: IoPortMetadata, const ID: u8, PortMode: Mode> Pin<Port, ID, PortMode> {
    /// Returns the number of the pin (for example, 12 for PC12).
    #[inline(always)]
    pub const fn id(&self) -> u8 {
        ID
    }

    /// Returns ID (uppercase letter) of the port of this pin (for example, 'C' for PC12).
    #[inline(always)]
    pub const fn port_id(&self) -> char {
        Port::ID
    }

    /// Transforms the pin into peripheral pin, giving control of it to selected peripheral.
    ///
    /// This function can be used to either change the mode of the pin, or change the peripheral
    /// controlling it.
    ///
    /// # Parameters
    /// * `peripheral` - Peripheral that will control the pin.
    pub fn into_peripheral_pin(mut self, peripheral: Peripheral) -> Pin<Port, ID, PeripheralMode> {
        self.select_peripheral(peripheral);

        // Give control to the peripheral
        self.registers_ref()
            .pdr
            .write(|w| unsafe { w.bits(self.pin_mask()) });

        Pin::transform(self)
    }

    /// Returns current logic state on pin's I/O line.
    pub fn state(&self) -> bool {
        self.is_pin_bit_set(self.registers_ref().pdsr.read().bits())
    }

    /// Returns true if pin is currently controlled by peripheral.
    pub fn is_peripheral_controlled(&self) -> bool {
        !self.is_pio_controlled()
    }

    /// Returns true if pin is currently controlled by PIO controller.
    pub fn is_pio_controlled(&self) -> bool {
        self.is_pin_bit_set(self.registers_ref().psr.read().bits())
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
    /// Implementation of this type makes sure that pins always access it's own bit, and doesn't access
    /// nor modify bits for other pins, therefore sharing this register block should be safe, as long
    /// as all pin types have correct, unique IDs specified by generic parameter `N`.
    pub(super) const fn registers_ref(&self) -> &RegisterBlock {
        unsafe { &*Port::REGISTERS }
    }

    /// Returns register mask for current pin.
    ///
    /// # Safety
    /// This function will panic if pin was created with invalid ID, enforcing safe design of this type.
    /// This makes it safe to use with `bits` method of register writer, as it guarantees that returned
    /// mask will always point to a valid bit in 32-bit PIO register.
    pub(super) const fn pin_mask(&self) -> u32 {
        if ID > 31 {
            panic!("invalid pin number, valid range is (0..=31)")
        }

        1u32 << ID
    }

    /// Helper function that checks whether the bit representing current pin in
    /// provided register's value is set.
    pub(super) const fn is_pin_bit_set(&self, register_value: u32) -> bool {
        (register_value & self.pin_mask()) != 0
    }

    /// Transform the pin into a type with different mode.
    ///
    /// This is a helper function that allows to reduce state transition boilerplate to minimum.
    ///
    /// Rust compiler can deduce `Self` from function's return type, and transformation is basically a
    /// no-op, so no code should be generated from this. This function is only to signalize the compiler
    /// that we really want to change the type of current pin.
    ///
    /// # Parameters
    /// * `_pin` - Pin to be transformed.
    const fn transform<NewMode: Mode>(_pin: Pin<Port, ID, NewMode>) -> Self {
        Self {
            _port_meta: PhantomData,
            _mode: PhantomData,
            _disable_send_and_sync: PhantomData,
        }
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

    /// Changes the peripheral that controls the pin.
    /// This change has no effect unless pin's control is switched to peripheral.
    ///
    /// # Parameters
    /// * `peripheral` - Peripheral that will control the pin.
    pub(super) fn select_peripheral(&mut self, peripheral: Peripheral) {
        // Original values of peripheral select registers with cleared bit representing
        // current pin, which will be set later (if needed).
        let mut select_registers = (
            self.registers_ref().abcdsr[0].read().bits() & (!self.pin_mask()),
            self.registers_ref().abcdsr[1].read().bits() & (!self.pin_mask()),
        );

        match peripheral {
            Peripheral::A => {
                // Peripheral A: (0, 0) in abcdsr 0/1 registers.
            }
            Peripheral::B => {
                // Peripheral B: (1, 0) in abcdsr 0/1 registers.
                select_registers.0 |= self.pin_mask();
            }
            Peripheral::C => {
                // Peripheral C: (0, 1) in abcdsr 0/1 registers.
                select_registers.1 |= self.pin_mask();
            }
            Peripheral::D => {
                // Peripheral D: (1, 1) in abcdsr 0/1 registers.
                select_registers.0 |= self.pin_mask();
                select_registers.1 |= self.pin_mask();
            }
        }

        // Safety: this is safe, because we're only modifying bit representing current pin.
        // This is not thread-safe, as it's not an atomic operation, but this type does not
        // implement `Sync`, so we're not breaking any invariants.
        self.registers_ref().abcdsr[0].write(|w| unsafe { w.bits(select_registers.0) });
        self.registers_ref().abcdsr[1].write(|w| unsafe { w.bits(select_registers.1) });
    }
}
