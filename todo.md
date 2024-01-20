## Text analysis for the generated assembly code
### Removing patterns like
- Move register content in the same destination register
  - ```nasm
    mov eax, eax
    ```
- Move something in a temp register, just to move it back
  - ```nasm
    movq rax, xmm7
    movq xmm7, rax
    ```
- Move something in a register, and then move it in the same register again
  - ```nasm
    movd eax, xmm0
    mov eax, DWORD [rbp - 4]
    ```

- Move something twice in the same register
  - ```nasm
    mov rax, rax
    mov rax, rax
    ```