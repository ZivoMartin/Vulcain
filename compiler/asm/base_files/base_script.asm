%include "asm/functions.asm"

global _start


section .text

_operation:

    cmp r12b, byte[_addi]
    je _addi_op
    cmp r12b, byte[_soustr]
    je _soustr_op
    cmp r12b, byte[_multip]
    je _multip_op
    cmp r12b, byte[_divis]
    je _divis_op
    cmp r12b, byte[_modulo]
    je _modulo_op

    cmp r12b, byte[_inf]
    je _inf_op
    cmp r12b, byte[_sup]
    je _sup_op
    cmp r12b, byte[_equal]
    je _equal_op
    cmp r12b, byte[_inf_equal]
    je _inf_equal_op
    cmp r12b, byte[_sup_equal]
    je _sup_equal_op
    cmp r12b, byte[_and]
    je _and_op
    cmp r12b, byte[_or]
    je _or_op
    cmp r12b, byte[_diff]
    je _diff_op

    _addi_op:
        add r10, r11
        mov rax, r10
        ret
    _soustr_op:
        sub r10, r11
        mov rax, r10
        ret
    _multip_op:
        mov rax, r10
        mul r11
        ret
    _divis_op:
        xor rdx, rdx
        mov rax, r10
        idiv r11
        ret

    _modulo_op:
        xor rdx, rdx
        mov rax, r10
        idiv r11
        mov rax, rdx
        ret 
    
    _inf_op:
        cmp r10, r11
        jl true
        jmp false

    _sup_op:
        cmp r10, r11
        jg true
        jmp false

    _inf_equal_op:
        cmp r10, r11
        jl true
        je true
        jmp false

    _sup_equal_op:
        cmp r10, r11
        jg true
        je true
        jmp false
    

    _equal_op:
        cmp r10, r11
        je true
        jmp false

    _diff_op:
        cmp r10, r11
        jne true
        jmp false

    _or_op:
        and r10, r10
        jne true
        and r11, r11
        jne true
        jmp false

    _and_op:
        and r10, r10
        je false
        and r10, r10
        je false
        jmp true


    true:
        mov rax, 1
        ret

    false:
        mov rax, 0
        ret
        
_invalid_address:
    mov rax, 1
    mov rdi, 1
    mov rsi, _seg_fault_msg
    mov rdx, _size_seg_fault_msg
    syscall
    exit 1

_start: