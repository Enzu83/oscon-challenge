use crate::num::Number;

#[derive(Debug)]
pub enum INSTRUCTION {
    HALT,
    SET(Number, Number),
    PUSH(Number),
    POP(Number),
    EQ(Number, Number, Number),
    GT(Number, Number, Number),
    JMP(Number),
    JT(Number, Number),
    JF(Number, Number),
    ADD(Number, Number, Number),
    MULT(Number, Number, Number),
    MOD(Number, Number, Number),
    AND(Number, Number, Number),
    OR(Number, Number, Number),
    NOT(Number, Number),
    RMEM(Number, Number),
    WMEM(Number, Number),
    CALL(Number),
    RET,
    OUT(Number),
    IN(Number),
    NOOP,
}
