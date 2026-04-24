/* Memory layout for SAMV71Q21 - SRAM-only execution
 *
 * SAMV71Q21 has 384 KB of SRAM (0x20400000–0x2045FFFF).
 * All application code, rodata, data, and stack reside in SRAM.
 */
MEMORY
{
    RAM (rwx) : ORIGIN = 0x20411000, LENGTH = 0x0004B000  /* This assumes that some part of SRAM is used by other application */
}
