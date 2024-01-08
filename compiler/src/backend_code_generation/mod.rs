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
        Expr::Number(value) => format!("mov eax, {}", value),
        Expr::BinOp(left, op, right) => {
            //let left_code = generate_asm_single(left);
            let right_code = generate_asm_single(right);

            match op {
                Operator::Add => format!("add eax, {}", right_code),
                Operator::Subtract => format!("sub eax, {}", right_code),
                Operator::Multiply => format!("imul eax, {}", right_code),
                Operator::Divide => format!("idiv eax, {}", right_code),
                Operator::Exp => {
                    format!("push rdi\nmov rdi, {}\npush rsi\ncall pow\npop rsi\npop rdi", right_code)
                }
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