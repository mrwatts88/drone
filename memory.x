MEMORY
{
  /* NOTE K = KiB = 1024 bytes */
  FLASH : ORIGIN = 0x08000000, LENGTH = 512K
  RAM   : ORIGIN = 0x20000000, LENGTH = 128K
}

/* Stack grows downward from the top of RAM */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);

