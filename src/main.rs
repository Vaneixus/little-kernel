extern "C" fn main() {
}

// Code pour initialiser le tout.
unsafe {
    !asm(
        ".section .init", // commencer ou le processeur du Pi attends du code.
        ".globl _start:", // necessaire pour que le compilateur de GNU ne porte pas de plaginte.
        "_start:",
        sym main,
    );
}
