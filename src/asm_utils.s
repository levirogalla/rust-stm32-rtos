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
	tst lr, 4 //TeST the 3rd bit in LR (4 is 0b1000, so its 3rd bit is 1)
	ite eq // tbh i don't really understand this instruction, but it does a condtional check to see if we are using msp or psp, then passes it to my function
	mrseq r0, msp
	mrsne r0, psp
    b SVCall_Handler

.global PendSV
.thumb_func
PendSV: // assume the user was running in thread mode, so we can use psp

    // save current threads registers to the stack
    mrs r0, psp // buffer psp into r0 for the next insturction
    stmdb r0!, {r4-r11} // store the registers r4-r11 on the stack, this is needed so they can be restored later, will store lowest register at the lowest address
    msr psp, r0 // update the process stack pointer with the new value


    // call rust function to run the more complex logic of finding the next task/thread to run
    bl PendSV_Handler 

    // restore next threads state, the scratch registers will be automatically restored by the return instruction
    mrs r0, psp
    ldmia r0!, {r4-r11} // load the registers r4-r11 from the stack, this is needed so that the registers are restored to their previous values
    msr psp, r0 // update the process stack pointer with the new value, this is needed so that the stack pointer points to the correct location
    mov lr, #0xFFFFFFFD // Set the return address to the thread mode, this is needed so that we can return to the thread mode after the function call
    bx lr



.global initial_context_switch
.thumb_func
initial_context_switch:
    mov lr, #0xFFFFFFFD //Set the return address to the thread mode
    ldmia r0!, {r4-r11} //Load the registers from the stack
    msr psp, r0 //Set the process stack pointer to the value in r0, the first argument
    bx lr //Return to the thread mode


// saving callee registers
    ; 
    ; msr psp, r0 // update the process stack pointer with the new value, this is needed so that the stack pointer points to the correct location