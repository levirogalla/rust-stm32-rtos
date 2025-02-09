.syntax unified
.thumb

@ Function to toggle the LED (PA5) and return the previous state of PA5
.global toggle_led
.thumb_func
toggle_led:
    @ Load GPIOA_ODR address (0x40020014)
    movw r0, #0x0014       @ Lower 16 bits of address
    movt r0, #0x4002       @ Upper 16 bits

    @ Load current ODR value into r1
    ldr  r1, [r0]

    @ Save the previous state of PA5 (bit 5) into r2
    mov  r2, r1            @ Copy ODR value to r2
    and  r2, r2, #(1 << 5) @ Isolate bit 5 (PA5)

    @ Toggle PA5 (bit 5)
    eors r1, r1, #(1 << 5) @ XOR bit 5 (toggle)
    str  r1, [r0]          @ Store back to ODR

    @ Return the previous state of PA5 (r2)
    mov  r0, r2            @ Move r2 (previous state) into r0 for return
    bx   lr                @ Return to caller