MEMORY {
	csr : ORIGIN = 0xe0000000, LENGTH = 0x00010000
	sram : ORIGIN = 0x10000000, LENGTH = 0x00020000
	spiflash : ORIGIN = 0x20000000, LENGTH = 0x01000000
	rom : ORIGIN = 0x20040000, LENGTH = 0x00fc0000
}

REGION_ALIAS("REGION_TEXT", spiflash);
REGION_ALIAS("REGION_RODATA", spiflash);
REGION_ALIAS("REGION_DATA", sram);
REGION_ALIAS("REGION_BSS", sram);
REGION_ALIAS("REGION_HEAP", sram);
REGION_ALIAS("REGION_STACK", sram);

/* CPU reset location. */
_stext = 0x20040000;
