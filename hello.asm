.global _main
.align 4

_main:
  mov X0, #0 ; Use 0 return code
  mov X16, #1 ; Unix exit system call
  svc #0x42 ; call kernel to end the program

