/* memory.x */
MEMORY
{
    FLASH : ORIGIN = 0x08000000, LENGTH = 256K
    RAM : ORIGIN = 0x20000000, LENGTH = 32K
}

_stack_start = ORIGIN(RAM) + LENGTH(RAM);