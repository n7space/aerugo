/* Linker script for SAMV71Q21 - SRAM-only execution
 *
 * This simplified linker script places all sections (code, rodata, data, bss, stack)
 * entirely in RAM. No flash memory is used.
 *
 * The memory.x file defines a single RAM region covering all 384 KB of SRAM.
 */

INCLUDE memory.x
INCLUDE device.x

/* # Entry point = reset vector */
EXTERN(__RESET_VECTOR);
EXTERN(Reset);
ENTRY(Reset);

/* # Exception vectors */
EXTERN(__EXCEPTIONS);
EXTERN(DefaultHandler);

PROVIDE(NonMaskableInt = DefaultHandler);
EXTERN(HardFaultTrampoline);
PROVIDE(MemoryManagement = DefaultHandler);
PROVIDE(BusFault = DefaultHandler);
PROVIDE(UsageFault = DefaultHandler);
PROVIDE(SecureFault = DefaultHandler);
PROVIDE(SVCall = DefaultHandler);
PROVIDE(DebugMonitor = DefaultHandler);
PROVIDE(PendSV = DefaultHandler);
PROVIDE(SysTick = DefaultHandler);

PROVIDE(DefaultHandler = DefaultHandler_);
PROVIDE(HardFault = HardFault_);

/* # Interrupt vectors */
EXTERN(__INTERRUPTS);

/* # Pre-initialization function */
PROVIDE(__pre_init = DefaultPreInit);

/* # Sections */
SECTIONS
{
  PROVIDE(_ram_start = ORIGIN(RAM));
  PROVIDE(_ram_end = ORIGIN(RAM) + LENGTH(RAM));
  PROVIDE(_stack_start = _ram_end);

  /* ## Vector table at start of RAM */
  .vector_table ORIGIN(RAM) :
  {
    __vector_table = .;
    LONG(_stack_start & 0xFFFFFFF8);
    KEEP(*(.vector_table.reset_vector));
    __exceptions = .;
    KEEP(*(.vector_table.exceptions));
    __eexceptions = .;
    KEEP(*(.vector_table.interrupts));
  } > RAM

  PROVIDE(_stext = ADDR(.vector_table) + SIZEOF(.vector_table));

  /* ## .text section */
  .text _stext :
  {
    __stext = .;
    *(.Reset);
    *(.text .text.*);
    *(.HardFaultTrampoline);
    *(.HardFault.*);
    . = ALIGN(4);
    __etext = .;
  } > RAM

  /* ## .rodata section */
  .rodata : ALIGN(4)
  {
    . = ALIGN(4);
    __srodata = .;
    *(.rodata .rodata.*);
    . = ALIGN(4);
    __erodata = .;
  } > RAM

  /* ## .data section (entirely in RAM, LMA==VMA) */
  .data : ALIGN(4)
  {
    . = ALIGN(4);
    __sdata = .;
    *(.data .data.*);
    . = ALIGN(4);
  } > RAM
  . = ALIGN(4);
  __edata = .;

  /* LMA of .data is same as VMA since everything is in RAM */
  __sidata = LOADADDR(.data);

  /* ## .bss section */
  .bss (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    __sbss = .;
    *(.bss .bss.*);
    *(COMMON);
    . = ALIGN(4);
  } > RAM
  . = ALIGN(4);
  __ebss = .;

  /* ## .uninit section */
  .uninit (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    __suninit = .;
    *(.uninit .uninit.*);
    . = ALIGN(4);
    __euninit = .;
  } > RAM

  /* Heap and stack placement */
  PROVIDE(__sheap = __euninit);
  PROVIDE(_stack_end = __euninit);

  /* Discarded sections */
  /DISCARD/ :
  {
    *(.ARM.exidx);
    *(.ARM.exidx.*);
    *(.ARM.extab.*);
  }
}

/* # Alignment checks */
ASSERT(ORIGIN(RAM) % 4 == 0, "RAM region must be 4-byte aligned");
ASSERT(__sdata % 4 == 0 && __edata % 4 == 0, ".data must be 4-byte aligned");
ASSERT(__sidata % 4 == 0, "LMA of .data must be 4-byte aligned");
ASSERT(__sbss % 4 == 0 && __ebss % 4 == 0, ".bss must be 4-byte aligned");
ASSERT(__sheap % 4 == 0, "heap start must be 4-byte aligned");
ASSERT(_stack_start % 8 == 0, "stack start must be 8-byte aligned");
ASSERT(_stack_end % 4 == 0, "stack end must be 4-byte aligned");
ASSERT(_stack_start >= _stack_end, "stack end must be below stack start");

/* # Position checks */
ASSERT(__exceptions == ADDR(.vector_table) + 0x8, "reset vector is missing");
ASSERT(__eexceptions == ADDR(.vector_table) + 0x40, "exception vectors are missing");
ASSERT(SIZEOF(.vector_table) > 0x40, "interrupt vectors are missing");
ASSERT(ADDR(.vector_table) + SIZEOF(.vector_table) <= _stext, ".text must be after .vector_table");
ASSERT(_stext > ORIGIN(RAM) && _stext < ORIGIN(RAM) + LENGTH(RAM), ".text must be in RAM");

/* # Other checks */
ASSERT(SIZEOF(.got) == 0, ".got section detected (dynamic relocations not supported)");
ASSERT(SIZEOF(.vector_table) <= 0x400, "too many interrupt handlers");
