use crate::parser::Expr;
use crate::parser::Operator;

pub fn generate_asm(ast: &[Expr]) -> String {
    let pow_function: String = generate_pow_function();
    let main_code: String = ast
        .iter()
        .map(|expr: &Expr| generate_asm_single(expr))
        .collect::<Vec<_>>()
        .join("\n");

    format!("{}\n\n{}", pow_function, main_code)
}

fn generate_asm_single(expr: &Expr) -> String {
    match expr {
        Expr::Number(value) => format!("push {}", value),
        Expr::BinOp(_left, op, right) => {
            let right_code = generate_asm_single(right);
            match op {
                Operator::Add => format!("pop rdi\nadd rdi, {}", right_code),
                Operator::Subtract => format!("pop rdi\nsub rdi, {}", right_code),
                Operator::Multiply => format!("pop rdi\nimul rdi, {}", right_code),
                Operator::Divide => format!("pop rdi\nidiv rdi, {}", right_code),
                Operator::Exp => {
                    println!("RIGHT CODE: {}", right_code);
                    format!("pop rdi\n{}\ncall pow", right_code)
                }
            }
        }
        Expr::Paren(inner_expr) => generate_asm_single(inner_expr), // Handle Paren expression
    }
}

fn generate_pow_function() -> String {
    "
    global pow
    section .text

    pow:
        ; Parameters: rdi = base, rsi = exponent
        mov rax, 1 ; Initialize result to 1
        test rsi, rsi ; Check if exponent is zero
        jz pow_done ; If zero, jump to the end

    pow_loop:
        test rsi, 1 ; Check if exponent is odd
        jnz pow_odd ; If odd, jump to pow_odd

        ; Exponent is even
        imul rdi, rdi ; Square the base
        shr rsi, 1 ; Divide exponent by 2
        jmp pow_loop ; Continue the loop

    pow_odd:
        ; Exponent is odd
        imul rax, rdi ; Multiply result by the base
        imul rdi, rdi ; Square the base
        shr rsi, 1 ; Divide exponent by 2
        jmp pow_loop ; Continue the loop

    pow_done:
        ; Result is in rax
        ret
    "
    .to_string()
}
