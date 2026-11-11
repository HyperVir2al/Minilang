use std::collections::HashMap;

use crate::ast::{BinaryOp, Expr, Stmt};
use crate::value::Value;

pub struct Evaluator {
    variables: HashMap<String, Value>,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            variables: HashMap::new(),
        }
    }

    pub fn run(&mut self, program: &[Stmt]) -> Result<(), String> {
        for stmt in program {
            self.execute(stmt)?;
        }
        Ok(())
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Let(name, expr) => {
                let value = self.eval(expr)?;
                self.variables.insert(name.clone(), value);
                Ok(())
            }
            Stmt::Print(expr) => {
                let value = self.eval(expr)?;
                println!("{}", value);
                Ok(())
            }
            Stmt::If(condition, then_block, else_block) => {
                match self.eval(condition)? {
                    Value::Bool(true) => self.run(then_block),
                    Value::Bool(false) => match else_block {
                        Some(block) => self.run(block),
                        None => Ok(()),
                    },
                    other => Err(format!(
                        "Ошибка выполнения: условие if должно быть логическим значением, получено {}",
                        other.type_name()
                    )),
                }
            }
        }
    }

    fn eval(&self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Literal(value) => Ok(value.clone()),
            Expr::Variable(name) => self
                .variables
                .get(name)
                .cloned()
                .ok_or_else(|| format!("Ошибка выполнения: переменная '{}' не объявлена", name)),
            Expr::Negate(inner) => match self.eval(inner)? {
                Value::Number(n) => Ok(Value::Number(-n)),
                other => Err(format!(
                    "Ошибка выполнения: унарный минус нельзя применить к значению типа {}",
                    other.type_name()
                )),
            },
            Expr::Binary(left, op, right) => {
                let l = self.eval(left)?;
                let r = self.eval(right)?;
                self.apply(l, *op, r)
            }
        }
    }

    fn apply(&self, left: Value, op: BinaryOp, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => match op {
                BinaryOp::Add => Ok(Value::Number(a + b)),
                BinaryOp::Sub => Ok(Value::Number(a - b)),
                BinaryOp::Mul => Ok(Value::Number(a * b)),
                BinaryOp::Div => {
                    if b == 0.0 {
                        Err("Ошибка выполнения: деление на ноль".to_string())
                    } else {
                        Ok(Value::Number(a / b))
                    }
                }
                BinaryOp::Mod => {
                    if b == 0.0 {
                        Err("Ошибка выполнения: остаток от деления на ноль".to_string())
                    } else {
                        Ok(Value::Number(a % b))
                    }
                }
                BinaryOp::Eq => Ok(Value::Bool(a == b)),
                BinaryOp::NotEq => Ok(Value::Bool(a != b)),
                BinaryOp::Greater => Ok(Value::Bool(a > b)),
                BinaryOp::Less => Ok(Value::Bool(a < b)),
                BinaryOp::GreaterEq => Ok(Value::Bool(a >= b)),
                BinaryOp::LessEq => Ok(Value::Bool(a <= b)),
            },
            (Value::Bool(a), Value::Bool(b)) => match op {
                BinaryOp::Eq => Ok(Value::Bool(a == b)),
                BinaryOp::NotEq => Ok(Value::Bool(a != b)),
                _ => Err("Ошибка выполнения: эта операция не поддерживается для логических значений".to_string()),
            },
            (a, b) => Err(format!(
                "Ошибка выполнения: операция неприменима к типам {} и {}",
                a.type_name(),
                b.type_name()
            )),
        }
    }
}
