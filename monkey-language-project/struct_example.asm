; This assembly is targeted for the Windows Operating System
section .data
    .label0: db "%d", 10, "", 0
    .label1: db "%f", 10, "", 0
    .name_gustaf: db "Gustaf", 10, "", 0


struc Person
    .age        resb 4  ; 4 bytes
    .height     resb 4  ; 4 bytes
    .name       resb 8  ; 8 bytes
endstruc

segment .text
global main

extern printf

main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64

    mov dword [rsp + Person.age], 27
    mov dword [rsp + Person.height], __?float32?__(1.82)
    mov rax, .name_gustaf
    mov qword [rsp + Person.name], rax

    mov rcx, rsp
    call .print_person~void


    ; return 0
    mov eax, 0
    leave
    ret


.print_a_i32~void:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 32
    mov DWORD [rbp - 4], ecx
    mov rax, .label0
    push rax
    pop rcx
    mov edx, DWORD [rbp - 4]
    ; printf("%d\n", value)
    call printf
    leave
    ret

.printf_ptrstring_f32~void:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 32
    mov QWORD [rbp - 8], rcx
    movd DWORD [rbp - 12], xmm1
    mov eax, DWORD [rbp - 12]
    movd xmm7, eax
    cvtss2sd xmm7, xmm7
    movq rax, xmm7
    movq xmm0, rax
    movq rax, xmm0
    push rax
    mov rcx, QWORD [rbp - 8]
    pop rdx
    movq xmm1, rdx
    ; printf(format, (f64)value)
    call printf
    leave
    ret

.print_person~void:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    mov QWORD [rbp - 8], rcx
    mov rcx, QWORD [rbp - 8]

    mov eax, DWORD [rcx + Person.age]
    mov ecx, eax
    call .print_a_i32~void

    mov rcx, QWORD [rbp - 8]
    mov rcx, QWORD [rcx + Person.name]
    call printf


    mov rcx, QWORD [rbp - 8]
    mov edx, DWORD [rcx + Person.height]
    movd xmm1, DWORD [rcx + Person.height]
    mov rcx, .label1
    call .printf_ptrstring_f32~void
    leave
    ret