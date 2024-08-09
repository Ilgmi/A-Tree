
pub mod logical_operations;

use crate::predicates::EqOperation::{Equal, NotEqual};
use crate::predicates::OrdOperation::{Greater, GreaterEqual, Less, LessEqual};
use crate::predicates::SetOperation::{ElementOf, NotElementOf};
use std::cmp::Ordering;
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug, Copy, Clone)]
pub struct Double(f64);
impl Hash for Double{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_string().hash(state)
    }
}
impl PartialEq for Double{
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < 0.0001
    }
}

impl PartialOrd for Double{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let int_part_self = self.0 as i32;
        let int_part_other = other.0 as i32;
        int_part_self.partial_cmp(&int_part_other)
    }
}

#[derive(Hash, PartialEq, PartialOrd, Debug)]
pub enum Value{
    Int(i32),
    Double(Double),
    String(String),
    Bool(bool)
}

pub trait Predicate {
    fn id(&self) -> u64;
    fn evaluate(&self, value: &Value) -> bool;
}


#[derive(Hash)]
pub enum EqOperation{
    Equal,NotEqual
}

pub struct EqualPredicate {
    constant: Value,
    operation: EqOperation
}

impl EqualPredicate {
    pub fn new(constant: Value, operation: EqOperation) -> Self{
        Self{
            constant,
            operation
        }
    }
}

impl  Predicate for EqualPredicate {
    fn id(&self) -> u64 {
        let mut h = DefaultHasher::new();
        self.constant.hash(&mut h);
        self.operation.hash(&mut h);
        h.finish()
    }

    fn evaluate(&self, value: &Value) -> bool
    {
        match self.operation {
            EqOperation::Equal => {value.eq(&self.constant)}
            EqOperation::NotEqual => {value.ne(&self.constant)}
        }
    }
}

pub fn equal(value: Value) -> EqualPredicate{
    EqualPredicate::new(value, Equal)
}

pub fn not_equal(value: Value) -> EqualPredicate{
    EqualPredicate::new(value, NotEqual)
}


#[derive(Hash)]
pub enum OrdOperation{
    Greater,GreaterEqual,LessEqual,Less
}

pub struct OrdPredicate {
    constant: Value,
    operation: OrdOperation,
}

impl OrdPredicate{
    pub fn new(constant: Value, operation: OrdOperation) -> Self{
        Self{
            constant,
            operation
        }
    }
}

impl Predicate for OrdPredicate {
    fn id(&self) -> u64 {
        let mut h = DefaultHasher::new();
        self.constant.hash(&mut h);
        self.operation.hash(&mut h);
        h.finish()
    }

    fn evaluate(&self, value: &Value) -> bool {
        match self.operation {
            OrdOperation::Greater => {value.gt(&self.constant)}
            OrdOperation::GreaterEqual => {value.ge(&self.constant)}
            OrdOperation::LessEqual => {value.le(&self.constant)}
            OrdOperation::Less => {value.lt(&self.constant)}
        }
    }
}

pub fn greater(value: Value) -> OrdPredicate{
    OrdPredicate::new(value, Greater)
}

pub fn greater_equal(value: Value) -> OrdPredicate{
    OrdPredicate::new(value, GreaterEqual)
}

pub fn less_equal(value: Value) -> OrdPredicate{
    OrdPredicate::new(value, LessEqual)
}

pub fn less(value: Value) -> OrdPredicate{
    OrdPredicate::new(value, Less)
}

pub enum SetOperation{
    ElementOf, NotElementOf
}

pub struct SetPredicate{
    constants: Vec<Value>,
    operation: SetOperation
}

impl SetPredicate{
    pub fn new(constants: Vec<Value>, operation: SetOperation) -> Self{
        Self{
            constants,
            operation
        }
    }

    pub fn push(&mut self, value: Value){
        self.constants.push(value)
    }
}

impl Predicate for SetPredicate{
    fn id(&self) -> u64 {
        let mut h = DefaultHasher::new();
        for constant in &self.constants {
            constant.hash(&mut h)
        }
        h.finish()
    }

    fn evaluate(&self, value: &Value) -> bool {
        match self.operation {
            SetOperation::ElementOf => {self.constants.contains(&value)}
            SetOperation::NotElementOf => {!self.constants.contains(&value)}
        }
    }
}

pub fn element_of(values: Vec<Value>) -> SetPredicate{
    SetPredicate::new(values, ElementOf)
}

pub fn not_element_of(values: Vec<Value>) -> SetPredicate{
    SetPredicate::new(values, NotElementOf)
}

pub struct BetweenPredicate {
    start_constant: Value,
    end_constant: Value,
}

impl BetweenPredicate{
    fn new(start_constant: Value, end_constant: Value) -> Self{
        Self{
            start_constant,
            end_constant
        }
    }
}

impl Predicate for BetweenPredicate{
    fn id(&self) -> u64 {
        let mut h = DefaultHasher::new();
        self.start_constant.hash(&mut h);
        self.end_constant.hash(&mut h);
        h.finish()
    }

    fn evaluate(&self, value: &Value) -> bool {
        value.ge(&self.start_constant) && value.le(&self.end_constant)
    }
}

pub fn between(start: Value, end: Value) -> BetweenPredicate{
    BetweenPredicate::new(start, end)
}






#[cfg(test)]
mod tests{
    use super::*;
    use crate::predicates::Value::Bool;
    use crate::predicates::Value::Int;

    #[test]
    fn not_equal_evaluation_for_same_value_is_false(){

        let values = vec![
            (Int(10), Int(10)), (Value::Double(Double(10.0)),Value::Double(Double(10.0))),
            (Value::String(String::from("10")),Value::String(String::from("10"))),
            (Bool(true),Bool(true))
        ];
        for value in values {
            assert!(!not_equal(value.0).evaluate(&value.1))
        }

    }

    #[test]
    fn equal_evaluation_for_not_the_same_value_is_true(){
        let values = vec![
            (Int(10), Int(11)), (Int(10), Value::Double(Double(10.0))), (Int(10), Value::String(String::from("10"))), (Int(10), Value::Bool(true)),
            (Value::Double(Double(10.0)), Value::Double(Double(11.0))), (Value::Double(Double(10.0)), Int(10)), (Value::Double(Double(10.0)), Value::String(String::from("10"))), (Value::Double(Double(10.0)), Value::Bool(true)),
            (Value::String(String::from("10")), Value::String(String::from("11"))),(Value::String(String::from("10")), Value::Double(Double(10.0))), (Value::String(String::from("10")), Int(10)), (Value::String(String::from("10")), Value::Bool(true)),
            (Value::Bool(true), Value::Bool(false)), (Value::Bool(true), Value::Double(Double(10.0))), (Value::Bool(true), Value::String(String::from("10"))), (Value::Bool(true), Int(10)),
        ];
        for value in values {
            assert!(not_equal(value.0).evaluate(&value.1))
        }
    }

    #[test]
    fn not_equal_evaluation_for_same_value_is_correct(){

        let values = vec![
            (Int(10), Int(10)), (Value::Double(Double(10.0)),Value::Double(Double(10.0))),
            (Value::String(String::from("10")),Value::String(String::from("10"))),
            (Bool(true),Bool(true))
        ];
        for value in values {
            assert!(equal(value.0).evaluate(&value.1))
        }

    }

    #[test]
    fn not_equal_evaluation_for_not_the_same_value_is_not_correct(){
        let values = vec![
            (Int(10), Value::Double(Double(10.0))), (Int(10), Value::String(String::from("10"))), (Int(10), Value::Bool(true)),
            // (Value::Double(Double(10.0)), Int(10)), (Value::Double(Double(10.0)), Value::String(String::from("10"))), (Value::Double(Double(10.0)), Value::Bool(true)),
            // (Value::String(String::from("10")), Value::Double(Double(10.0))), (Value::String(String::from("10")), Int(10)), (Value::String(String::from("10")), Value::Bool(true)),
            // (Value::Bool(true), Value::Double(Double(10.0))), (Value::Bool(true), Value::String(String::from("10"))), (Value::Bool(true), Int(10)),
        ];
        for value in values {
            println!("Testing {:?} and {:?}", &value.0, &value.1);
            assert_eq!(not_equal(value.0).evaluate(&value.1), true)
        }
    }

}