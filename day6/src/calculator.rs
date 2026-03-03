mod operations {
    use std::{
        ops::{Add, Mul},
        process::Output,
    };

    fn sum<T: Add<Output = T>>(lhs: T, rhs: T) -> T {
        lhs + rhs
    }

    fn multiply<T: Mul<Output = T>>(lhs: T, rhs: T) -> T {
        lhs * rhs
    }
}

#[derive(Debug)]
pub enum OperationType {
    Sum,
    Multiplication,
}

impl From<&str> for OperationType {
    fn from(value: &str) -> Self {
        match value {
            "+" => Self::Sum,
            "*" => Self::Multiplication,
            _ => panic!("symbol not considered in OperationType enum"),
        }
    }
}

pub struct Calculator {
    operands: Vec<Vec<u64>>,
    operations: Vec<OperationType>,
}

impl Calculator {
    pub fn new(operands: Vec<Vec<u64>>, operations: Vec<OperationType>) -> Self {
        Self {
            operands,
            operations,
        }
    }

    pub fn resolve(&self) -> u64 {
        let mut total = 0;
        for (idx, o) in self.operations.iter().enumerate() {
            let mut partial_result = match o {
                OperationType::Sum => 0,
                OperationType::Multiplication => 1,
            };
            for r in 0..self.operands.len() {
                match o {
                    OperationType::Sum => partial_result += self.operands[r][idx],
                    OperationType::Multiplication => partial_result *= self.operands[r][idx],
                }
            }
            total += partial_result;
        }

        total
    }

    pub fn resolve_cephalopod(operands: Vec<Vec<u64>>, operations: Vec<OperationType>) -> u64 {
        let mut total = 0;
        operations
            .iter()
            .zip(operands)
            .for_each(|(op_type, operands)| match op_type {
                OperationType::Sum => total += operands.iter().sum::<u64>(),
                OperationType::Multiplication => total += operands.iter().product::<u64>(),
            });
        total
    }
}
