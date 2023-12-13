/* SRC: https://www.apriorit.com/dev-blog/696-kernel-embedded-development-for-raspberry-pi#q7 */
SECTIONS
{
    . = 0x80000;

    .text :
    {
        *(.text._start) *(.text*)
    }
}