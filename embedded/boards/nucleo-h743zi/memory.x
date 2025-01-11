MEMORY {
       FLASH : ORIGIN = 0x08000000, LENGTH = 2M
       
/*     RAM : ORIGIN = 0x20000000, LENGTH = 128k

       AXISRAM : ORIGIN = 0x24000000, LENGTH = 512k */

/* */  RAM : ORIGIN = 0x24000000, LENGTH = 512k /* */

       SRAM1 : ORIGIN = 0x30000000, LENGTH = 128K
       SRAM2 : ORIGIN = 0x30020000, LENGTH = 128K
       SRAM3 : ORIGIN = 0x30040000, LENGTH = 32K
       SRAM4 : ORIGIN = 0x38000000, LENGTH = 64K

       BSRAM : ORIGIN = 0x38800000, LENGTH = 4K

       ITCM  : ORIGIN = 0x00000000, LENGTH = 64K
}

_stack_start = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS {
/*
  .axisram (NOLOAD) : ALIGN(8) {
    *(.axisram .axisram.*);
    . = ALIGN(8);
    } > AXISRAM
  */
  
  .sram3 (NOLOAD) : ALIGN(4) {
    *(.sram3 .sram3.*);
    . = ALIGN(4);
    } > SRAM3
    
  .sram4 (NOLOAD) : ALIGN(4) {
    *(.sram4 .sram4.*);
    . = ALIGN(4);
    } > SRAM4
};

