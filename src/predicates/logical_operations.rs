use std::ops::Not as OpsNot;
use crate::predicates::{Predicate, Value};

struct And
{
    lhs: Box<dyn Predicate>,
    rhs: Box<dyn Predicate>
}

impl And {
    fn new(lhs: Box<dyn Predicate>, rhs: Box<dyn Predicate>) -> Self{
        Self{
            lhs,
            rhs,
        }
    }
}

impl Predicate for And
{
    fn id(&self) -> u64 {
        self.lhs.id().overflowing_mul(self.rhs.id()).0
    }

    fn evaluate(&self, value: &Value) -> bool {
        self.lhs.evaluate(value) && self.rhs.evaluate(value)
    }
}

pub struct Ands
{
    predicates: Vec<Box<dyn Predicate>>
}

impl Ands {
    fn new() -> Self{
        Self{
            predicates: vec![]
        }
    }
}

impl Ands
{
    fn with(&mut self, other: impl Predicate + 'static){
        self.predicates.push(Box::new(other))
    }
}

impl Predicate for Ands
{
    fn id(&self) -> u64 {
        let mut id: u64 = 1;
        for predicate in &self.predicates {
            let mul = id.overflowing_mul(predicate.id());
            id = mul.0
        }
        id
    }

    fn evaluate(&self, value: &Value) -> bool {
        for predicate in &self.predicates {
            if !predicate.evaluate(value) {
                return false;
            }
        }
        return true;
    }
}


struct Or
{
    lhs: Box<dyn Predicate>,
    rhs: Box<dyn Predicate>
}

impl Or {
    fn new(lhs: Box<dyn Predicate>, rhs: Box<dyn Predicate>) -> Self{
        Self{
            lhs,
            rhs,
        }
    }
}

impl Predicate for Or
{
    fn id(&self) -> u64 {
        self.lhs.id().overflowing_add(self.rhs.id()).0
    }

    fn evaluate(&self, value: &Value) -> bool {
        self.lhs.evaluate(value) || self.rhs.evaluate(value)
    }
}

struct Ors {
    predicates: Vec<Box<dyn Predicate>>
}

impl Ors {
    fn new() -> Self{
        Self{
            predicates: vec![]
        }
    }

    fn with(&mut self, predicate: impl Predicate + 'static){
        self.predicates.push(Box::new(predicate))
    }
}

impl Predicate for Ors {
    fn id(&self) -> u64 {
        let mut id:u64 = 0;
        for predicate in &self.predicates {
            id = id.overflowing_add(predicate.id()).0
        }
        id
    }

    fn evaluate(&self, value: &Value) -> bool {
        for predicate in &self.predicates {
            if predicate.evaluate(value) {
                return true;
            }
        }
        return false;
    }
}

struct Not
{
    pred: Box<dyn Predicate>,
}

impl Not {
    fn new(pred: Box<dyn Predicate>) -> Self{
        Self{
            pred
        }
    }
}

impl Predicate for Not
{
    fn id(&self) -> u64 {
        self.pred.id().not()
    }

    fn evaluate(&self, value: &Value) -> bool {
        self.pred.evaluate(value).not()
    }
}

pub trait PredicateOperationExt
where
    Self: Predicate + 'static
{
    fn and(self, other: impl Predicate + 'static) -> And
    where Self: Sized
    {
        And::new(Box::new(self), Box::new(other))
    }

    fn or(self, other: impl Predicate + 'static) -> Or
    where Self: Sized{
        Or::new(Box::new(self), Box::new(other))
    }

    fn not(self) -> Not
    where Self: Sized{
        Not::new(Box::new(self))
    }
}

impl <P> PredicateOperationExt for P
where P: Predicate + 'static
{
}


fn multiple_and() -> Ands {
    Ands::new()
}
