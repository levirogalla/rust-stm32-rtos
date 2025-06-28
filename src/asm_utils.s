.syntax unified //This lets us use C like comments!
.cpu cortex-m4 //Guess what this does
.thumb //Practically this only matters to the CPU, but it ensures that the correct types of instructions get included

; .global idleThread
; .thumb_func
; idleThread:
;   B .


.global SVCall
.thumb_func
SVCall:
    mov r0, sp // get location of where stack frame is created and pass it to rust, this can't be done in rust because the compiler adds a bunch of stuff on the stack ontop of this if we are making a call from msp
    b SVCall_Handler


.global initial_context_switch
.thumb_func
initial_context_switch:
    mov lr, #0xFFFFFFFD //Set the return address to the thread mode
    ldmia r0!, {r4-r11} //Load the registers from the stack
    msr psp, r0 //Set the process stack pointer to the value in r0, the first argument
    bx lr //Return to the thread mode