use std::{io, vec};

#[derive(Clone, Copy)]
enum Expression {
    Number(f64),
    Plus,
    Minus,
    Times,
    Div,
}

enum RNPError {
    InvalidOperand,
    TermsQuantityInvalid,
}

fn parse(expression: String) -> Result<f64, RNPError> {
    let mut stack: Vec<f64> = vec![];

    for op in expression.trim().split(" ") {
        let parsed_exp: Option<Expression> = match op {
            "+" => Some(Expression::Plus),
            "-" => Some(Expression::Minus),
            "*" => Some(Expression::Times),
            "/" => Some(Expression::Div),
            other => match other.parse::<f64>() {
                Ok(i) => Some(Expression::Number(i)),
                Err(_) => None,
            },
        };

        if let Some(exp) = parsed_exp {
            if let Expression::Number(i) = exp {
                stack.push(i);
            } else {
                let one = stack.pop();
                let two = stack.pop();

                if let (Some(one_op), Some(two_op)) = (one, two) {
                    let res = match exp {
                        Expression::Plus => one_op + two_op,
                        Expression::Minus => one_op - two_op,
                        Expression::Div => one_op / two_op,
                        Expression::Times => one_op * two_op,
                        _ => return Err(RNPError::InvalidOperand),
                    };

                    stack.push(res)
                } else {
                    return Err(RNPError::TermsQuantityInvalid);
                }
            }
        } else {
            return Err(RNPError::InvalidOperand);
        }
    }

    if stack.len() != 1 {
        return Err(RNPError::TermsQuantityInvalid);
    }

    return match stack.pop() {
        Some(i) => Ok(i),
        None => Err(RNPError::TermsQuantityInvalid),
    };
}

fn main() {
    println!("Insira a expressão RNP:");
    let mut exp = String::new();
    io::stdin()
        .read_line(&mut exp)
        .expect("Não foi possível ler a expressão");
    match parse(exp) {
        Ok(res) => println!("O resultado é {}", res),
        Err(e) => match e {
            RNPError::InvalidOperand => println!("Operador inválido"),
            RNPError::TermsQuantityInvalid => println!("Quantidade de termos errada"),
        },
    }
}
