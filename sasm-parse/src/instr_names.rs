macro_rules! define_instr_names {
    ($($name:ident),*) => {
        $(pub const $name: &str = stringify!($name);)*
    };
}

#[rustfmt::skip]
define_instr_names!(
    VAR,
    MOV,
    INC,
    DEC,
    DMP,
    ADD,
    SUB,
    MUL,
    DIV,
    POW,
    CMP,
    JNE,
    JEQ,
    JMP,
    RNV,
    RSV,
    RNG,
    PSH,
    POP,
    FMT,
    SAY,
    DIE
);
