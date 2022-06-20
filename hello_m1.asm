.global _main
.align 4

_main:
    mov X0, #1  ; arg[0] = 1 = STDOUT
    adr X1, str ; arg[1] = string to print
    mov X2, #17 ; arg[2] = the string's length
    mov X16, #4 ; Unix write system call
    svc #0x42   ; call kernel to output the string

    mov X0, #0  ; Use 0 return code
    mov X16, #1 ; Unix exit system call
    svc #0x42   ; call kernel to end the program

str: .ascii "Hello, M1 World!\n"
