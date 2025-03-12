.syntax unified //This lets us use C like comments!
.cpu cortex-m4 //Guess what this does
.thumb //Practically this only matters to the CPU, but it ensures that the correct types of instructions get included

; .global idleThread
; .thumb_func
; idleThread:
;   B .

.global runFirstThread //Running the first thread requires some special consideration, so it is its own function
.thumb_func
runFirstThread:
//Restore MSP since we have two things on there that won't go away
; POP {R7}
; POP {R7}

//Get ready for PSP
MRS R0, PSP
MOV LR, #0xFFFFFFFD
// LDMIA R0!,{R4-R11}
MSR PSP, R0
BX LR


.global SVCall
.thumb_func
SVCall:
    mov r0, sp // get location of where stack frame is created and pass it to rust, this can't be done in rust because the compiler adds a bunch of stuff on the stack ontop of this if we are making a call from msp
    b SVCall_Handler
