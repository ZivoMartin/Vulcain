section .data

    _multip: db '*'
    _soustr: db '-'
    _addi: db '+'
    _divis: db '/'
    _modulo: db '%'
    
    _chiffres: db '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
    _newline: db 10

section .bss
     _stack: resb 300000