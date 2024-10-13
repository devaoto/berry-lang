use std::collections::HashMap;
use crate::parser::{ Expr, BinOp };

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
}

pub struct Interpreter {
    symbol_table: HashMap<String, (bool, Value)>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            symbol_table: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::Binary(left, op, right) => {
                let left_val = self.interpret(left)?;
                let right_val = self.interpret(right)?;
                self.evaluate_binary_op(&left_val, op, &right_val)
            }
            Expr::Var(name) => {
                self.symbol_table
                    .get(name)
                    .map(|(_, value)| value.clone())
                    .ok_or_else(|| format!("Undefined variable: {}", name))
            }
            Expr::Assign(name, expr) => {
                if let Some((is_immut, _)) = self.symbol_table.get(name) {
                    if *is_immut {
                        return Err(format!("Cannot assign to constant variable: {}", name));
                    }
                } else {
                    return Err(format!("Variable not declared: {}", name));
                }
                let value = self.interpret(expr)?;
                self.symbol_table.insert(name.clone(), (false, value.clone()));
                Ok(value)
            }
            Expr::VarDecl(is_immut, name, expr) => {
                if self.symbol_table.contains_key(name) {
                    return Err(format!("Variable already declared: {}", name));
                }
                let value = self.interpret(expr)?;
                self.symbol_table.insert(name.clone(), (*is_immut, value.clone()));
                Ok(value)
            }
            Expr::Block(statements) => {
                let mut last_value = Value::Number(0.0);
                for stmt in statements {
                    last_value = self.interpret(stmt)?;
                }
                Ok(last_value)
            }
            Expr::PrintStmt(expr) => {
                let value = self.interpret(expr)?;
                match value {
                    Value::Number(n) => {
                        println!("{}", n);
                        Ok(Value::Number(n))
                    }
                }
            }
        }
    }

    fn evaluate_binary_op(&self, left: &Value, op: &BinOp, right: &Value) -> Result<Value, String> {
        match (left, op, right) {
            (Value::Number(l), BinOp::Plus, Value::Number(r)) => Ok(Value::Number(l + r)),
            (Value::Number(l), BinOp::Minus, Value::Number(r)) => Ok(Value::Number(l - r)),
            (Value::Number(l), BinOp::Multiply, Value::Number(r)) => Ok(Value::Number(l * r)),
            (Value::Number(l), BinOp::Divide, Value::Number(r)) => {
                if *r == 0.0 { Ok(Value::Number(*l)) } else { Ok(Value::Number(l / r)) }
            }
            (Value::Number(l), BinOp::Mod, Value::Number(r)) => {
                if *r == 0.0 { Ok(Value::Number(*l)) } else { Ok(Value::Number(l % r)) }
            }
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.symbol_table.get(name).map(|(_, value)| value)
    }
}
