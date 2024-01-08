use crate::parser::Expr;
use crate::parser::Operator;

//#[derive(Debug, Clone)]
/*enum Expr {
    Number(i64),
    BinOp(Box<Expr>, Operator, Box<Expr>)
}
*/

/*#[derive(Debug, Clone)]
enum Operator {
    Add,
    Subtract
}
*/

pub fn generate_asm(ast: &[Expr]) -> String {
    let pow_function = generate_pow_function();
    let main_code = ast
        .iter()
        .map(|expr| generate_asm_single(expr))
        .collect::<Vec<_>>()
        .join("\n");
    format!("{}\n\n{}", pow_function, main_code)
}

fn generate_asm_single(expr: &Expr) -> String {
    match expr {
        Expr::Number(value) => format!("mov aex, {}", value),
        Expr::BinOp(left, op, right) => {
            let left_code: String = generate_asm_single(left);
            let right_code: String = generate_asm_single(right);
            match op {
                Operator::Add => format!("{} \n add eax, {}", left_code, right_code),
                Operator::Subtract => format!("{} \n sub eax, {}", left_code, right_code),
                Operator::Multiply => format!("{} \n imul aex, {}", left_code, right_code),
                Operator::Divide => format!("{} \n idiv eax, {}", left_code, right_code),
                Operator::Exp => format!("{} \n push eax \n {} \n pop ebx \n call power", left_code, right_code)
            }
        }
    }
}

fn generate_pow_function() -> String {
    "
    global pow
    section .text

    pow:
        ; Parameters: rdi = base, rsi = exponent
        mov rax, 1 ; Initialize result to 1
        mov rcx, rsi ; Copy exponent to rcx

    pow_loop:
        cmp rcx, 0
        je pow_done ; Jump to the end if exponent is 0
        imul rax, rdi ; Multiply result by base
        dec rcx ; Decrement exponent
        jmp pow_loop ; Jump back to the loop

    pow_done:
        ; Result is in rax
        ret
    "
    .to_string()
}