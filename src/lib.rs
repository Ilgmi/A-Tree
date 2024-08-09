use std::cell::{RefCell, RefMut};
use std::collections::{HashMap, VecDeque};
use std::ops::{Add, Deref, DerefMut};
use std::sync::Arc;

use crate::LogOperation::{And, Or};
use crate::predicates::{BetweenPredicate, EqualPredicate, OrdPredicate, Predicate, SetPredicate, Value};

mod predicates;



enum Predicates{
    OrdPredicate(OrdPredicate),
    EqualPredicate(EqualPredicate),
    SetPredicate(SetPredicate),
    BetweenPredicate(BetweenPredicate),
}

#[derive(Debug, Clone)]
enum NodeType {
    LeafNodeType(LeafNode),
    InnerNodeType(InnerNode),
    RootNodeType(RootNode)
}

impl NodeType{
    fn new_leaf(node: LeafNode) -> ArcNodeLink{
        Arc::new(RefCell::new(NodeType::LeafNodeType(node)))
    }

    fn new_inner(node: InnerNode) -> ArcNodeLink{
        Arc::new(RefCell::new(NodeType::InnerNodeType(node)))
    }

    fn new_root(node: RootNode) -> ArcNodeLink{
        Arc::new(RefCell::new(NodeType::RootNodeType(node)))
    }
}

impl Node for NodeType{
    type Node = NodeType;


    fn get_id(&self) -> u64 {
        match self {
            NodeType::LeafNodeType(node) => {node.get_id()}
            NodeType::InnerNodeType(node) => {node.get_id()}
            NodeType::RootNodeType(node) => {node.get_id()}
        }
    }

    fn get_level(&self, level: u32) -> u32 {
        match self {
            NodeType::LeafNodeType(node) => {node.get_level(level)}
            NodeType::InnerNodeType(node) => {node.get_level(level)}
            NodeType::RootNodeType(node) => {node.get_level(level)}
        }
    }

    fn add_children(&mut self, node: Arc<RefCell<Self::Node>>) -> Option<Arc<RefCell<Self::Node>>> {
        match self {
            NodeType::LeafNodeType(n) => { n.add_children(node)}
            NodeType::InnerNodeType(n) => { n.add_children(node)}
            NodeType::RootNodeType(n) => { n.add_children(node)}
        }
    }


    fn get_children(&self) -> Option<&[Arc<RefCell<Self::Node>>]>{
        match self {
            NodeType::LeafNodeType(node) => {node.get_children()}
            NodeType::InnerNodeType(node) => {node.get_children()}
            NodeType::RootNodeType(node) => {node.get_children()}
        }
    }

    fn add_parent(&mut self, node: Arc<RefCell<Self::Node>>) -> Option<Arc<RefCell<Self::Node>>> {
        match self {
            NodeType::LeafNodeType(n) => { n.add_parent(node)}
            NodeType::InnerNodeType(n) => { n.add_parent(node)}
            NodeType::RootNodeType(n) => { n.add_parent(node)}
        }
    }

    fn get_parents(&self) -> Option<&[Arc<RefCell<Self::Node>>]> {
        match self {
            NodeType::LeafNodeType(node) => {node.get_parents()}
            NodeType::InnerNodeType(node) => {node.get_parents()}
            NodeType::RootNodeType(node) => {node.get_parents()}
        }
    }

    fn evaluate(&self) -> Option<bool> {
        match self {
            NodeType::LeafNodeType(node) => {node.evaluate()}
            NodeType::InnerNodeType(node) => {node.evaluate()}
            NodeType::RootNodeType(node) => {node.evaluate()}
        }
    }

    fn clean(&mut self) {
        match self {
            NodeType::LeafNodeType(node) => {node.clean()}
            NodeType::InnerNodeType(node) => {node.clean()}
            NodeType::RootNodeType(node) => {node.clean()}
        }
    }
}

#[derive(Debug,Clone)]
enum LogOperation{
    And,Or
}


trait Node{

    type Node;

    fn get_id(&self) -> u64;
    fn get_level(&self, level:u32) -> u32;

    fn add_children(&mut self, node: Arc<RefCell<Self::Node>>) -> Option<Arc<RefCell<Self::Node>>>;
    fn get_children(&self) -> Option<&[Arc<RefCell<Self::Node>>]>;

    fn add_parent(&mut self, node: Arc<RefCell<Self::Node>>) -> Option<Arc<RefCell<Self::Node>>>;
    fn get_parents(&self) -> Option<&[Arc<RefCell<Self::Node>>]>;

    fn evaluate(&self) -> Option<bool>;
    fn clean(&mut self);

}

type ArcNodeLink =  Arc<RefCell<NodeType>>;

#[derive(Debug, Clone)]
struct LeafNode{
    predicate_id: u64,
    parents: Vec<ArcNodeLink>,
    pub result: Option<bool>
}

impl LeafNode{
    fn new(predicate_id: u64) -> Self{
        Self{
            predicate_id,
            parents: vec![],
            result: None
        }
    }
}

impl Node for LeafNode{

    type Node = NodeType;


    fn get_id(&self) -> u64 {
        self.predicate_id
    }

    fn get_level(&self, level: u32) -> u32 {
        level.add(1)
    }

    fn add_children(&mut self, _: Arc<RefCell<Self::Node>>) -> Option<Arc<RefCell<Self::Node>>> {
        None
    }

    fn get_children(&self) -> Option<&[Arc<RefCell<Self::Node>>]> {
        None
    }

    fn add_parent(&mut self, node: Arc<RefCell<Self::Node>>) -> Option<Arc<RefCell<Self::Node>>>{
        let r = node.clone();
        self.parents.push(node);
        Some(r)
    }

    fn get_parents(&self) -> Option<&[Arc<RefCell<Self::Node>>]> {
        Some(self.parents.as_slice())
    }

    fn evaluate(&self) -> Option<bool> {
        self.result
    }

    fn clean(&mut self) {
        self.result = None
    }
}

#[derive(Debug, Clone)]
struct InnerNode{
    pub log_operation: LogOperation,
    parents: Vec<ArcNodeLink>,
    childrens: Vec<ArcNodeLink>,
    pub operands: Vec<Option<bool>>
}

impl InnerNode{
    fn new(log_operation: LogOperation) -> Self{
        Self{
            log_operation,
            parents: vec![],
            childrens: vec![],
            operands: vec![]
        }
    }

    fn and() -> Self {
        Self{
            log_operation: And,
            parents: vec![],
            childrens: vec![],
            operands: vec![]
        }
    }

    fn or() -> Self {
        Self{
            log_operation: Or,
            parents: vec![],
            childrens: vec![],
            operands: vec![]
        }
    }
}

impl Node for InnerNode{

    type Node = NodeType;
    fn get_id(&self) -> u64 {
        match self.log_operation {
            LogOperation::And => {
                self.childrens.iter().fold(0, |a, b|{a.overflowing_add(b.borrow().get_id()).0})
            }
            LogOperation::Or => {self.childrens.iter().fold(1, |a, b|{a.overflowing_mul(b.borrow().get_id()).0})}
        }
    }

    fn get_level(&self, level: u32) -> u32 {
        let mut max_level = 0;
        for node in &self.childrens {
            let level = node.borrow().get_level(level + 1);
            max_level = level.max(max_level);
        }
        max_level
    }

    fn add_children(&mut self, node: Arc<RefCell<Self::Node>>) -> Option<Arc<RefCell<Self::Node>>> {
        let r = node.clone();
        self.childrens.push(node);
        Some(r)
    }


    fn get_children(&self) -> Option<&[Arc<RefCell<Self::Node>>]> {
        Some(self.childrens.as_slice())
    }

    fn add_parent(&mut self, node: Arc<RefCell<Self::Node>>) -> Option<Arc<RefCell<Self::Node>>>{
        let r = node.clone();
        self.parents.push(node);
        Some(r)
    }

    fn get_parents(&self) -> Option<&[Arc<RefCell<Self::Node>>]> {
        Some(self.parents.as_slice())
    }

    fn evaluate(&self) -> Option<bool> {
        match self.log_operation {
            And => {
                let mut iter = self.operands.iter();
                let mut op1 = iter.next().unwrap().clone();
                while let Some(op2) = iter.next(){
                    match (op1, op2.clone()) {
                        (None, Some(true)) => {op1 = None}
                        (None, Some(false)) => {op1 = Some(false)}
                        (Some(true), None) => {op1 = None}
                        (Some(false), None) => {op1 = Some(false)}
                        (Some(val1), Some(val2)) => {op1 = Some(val1 && val2)}
                        (None, None) => { op1 = None }
                    }
                }
                op1
            }
            Or => {
                let mut iter = self.operands.iter();
                let mut op1 = iter.next().unwrap().clone();
                while let Some(op2) = iter.next(){
                    match (op1, op2.clone()) {
                        (None, Some(true)) => {op1 = Some(true)}
                        (None, Some(false)) => {op1 = None}
                        (Some(true), None) => {op1 = Some(true)}
                        (Some(false), None) => {op1 = None}
                        (Some(val1), Some(val2)) => {op1 = Some(val1 || val2)}
                        (None, None) => { op1 = None }
                    }
                }
                op1
            }
        }

    }

    fn clean(&mut self) {
        self.operands.clear()
    }
}

#[derive(Debug,Clone)]
struct RootNode{
    childrens: Vec<ArcNodeLink>,
    pub log_operation: LogOperation,
    pub operands: Vec<Option<bool>>
}

struct RootNodeBuilder{
    node: ArcNodeLink
}

impl RootNodeBuilder{

    fn and() -> Self{
        Self{
            node: Arc::new(RefCell::new(NodeType::RootNodeType(RootNode::new(And))))
        }
    }

    fn or() -> Self{
        Self{
            node: Arc::new(RefCell::new(NodeType::RootNodeType(RootNode::new(Or))))
        }
    }

    fn with_inner_node(&mut self, node: InnerNode) -> &mut Self{
        let mut node = node;
        node.add_parent(self.node.clone());
        self.node.borrow_mut().add_children(Arc::new(RefCell::new(NodeType::InnerNodeType(node))));
        self
    }

    fn with_leaf_node(&mut self, node: LeafNode) -> &mut Self{
        let mut node = node;
        node.add_parent(self.node.clone());
        self.node.borrow_mut().add_children(Arc::new(RefCell::new(NodeType::LeafNodeType(node))));
        self
    }
}

impl RootNode{
    fn new(log_operation: LogOperation) -> Self{
        Self{
            log_operation,
            childrens: vec![],
            operands: vec![]
        }
    }

    fn and() -> Self {
        Self{
            log_operation: And,
            childrens: vec![],
            operands: vec![]
        }
    }

    fn or() -> Self {
        Self{
            log_operation: Or,
            childrens: vec![],
            operands: vec![]
        }
    }

}


impl Node for RootNode{
    type Node = NodeType;


    fn get_id(&self) -> u64 {
        match self.log_operation {
            LogOperation::And => {
                self.childrens.iter().fold(0, |a, b|{a.overflowing_add(b.borrow().get_id()).0})
            }
            LogOperation::Or => {
                self.childrens.iter().fold(1, |a, b|{a.overflowing_mul(b.borrow().get_id()).0})
            }
        }
    }

    fn get_level(&self, level: u32) -> u32 {
        let mut max_level = 0;
        for node in &self.childrens {
            let level = node.borrow().get_level(level + 1);
            max_level = level.max(max_level);
        }
        max_level
    }

    fn add_children(&mut self, node: Arc<RefCell<Self::Node>>) -> Option<Arc<RefCell<Self::Node>>> {
        let r = node.clone();
        self.childrens.push(node);
        Some(r)
    }

    fn get_children(&self) -> Option<&[Arc<RefCell<Self::Node>>]> {
        Some(&self.childrens)
    }

    fn add_parent(&mut self, _: Arc<RefCell<Self::Node>>) -> Option<Arc<RefCell<Self::Node>>>{
        None
    }

    fn get_parents(&self) -> Option<&[Arc<RefCell<Self::Node>>]> {
        None
    }

    fn evaluate(&self) -> Option<bool> {
        match self.log_operation {
            And => {
                let mut iter = self.operands.iter();
                let mut op1 = iter.next().unwrap().clone();
                while let Some(op2) = iter.next(){
                    match (op1, op2.clone()) {
                        (None, Some(true)) => {op1 = None}
                        (None, Some(false)) => {op1 = Some(false)}
                        (Some(true), None) => {op1 = None}
                        (Some(false), None) => {op1 = Some(false)}
                        (Some(val1), Some(val2)) => {op1 = Some(val1 && val2)}
                        (None, None) => { op1 = None }
                    }
                }
                op1
            }
            Or => {
                let mut iter = self.operands.iter();
                let mut op1 = iter.next().unwrap().clone();
                while let Some(op2) = iter.next(){
                    match (op1, op2.clone()) {
                        (None, Some(true)) => {op1 = Some(true)}
                        (None, Some(false)) => {op1 = None}
                        (Some(true), None) => {op1 = Some(true)}
                        (Some(false), None) => {op1 = None}
                        (Some(val1), Some(val2)) => {op1 = Some(val1 || val2)}
                        (None, None) => { op1 = None }
                    }
                }
                op1
            }
        }
    }

    fn clean(&mut self) {
        self.operands.clear()
    }
}


fn add_children(node: &mut ArcNodeLink, children: &mut ArcNodeLink){
    children.borrow_mut().add_parent(node.deref().clone());
    node.borrow_mut().add_children(children.deref().clone());
}

struct PredResult{
    pub id: u64,
    pub result: Option<bool>
}


struct ATree{

    hash_to_node: HashMap<u64, ArcNodeLink>

}

impl ATree{

    fn new() -> Self{
        ATree{
            hash_to_node: HashMap::new()
        }
    }

    fn len(&self) -> usize{
        self.hash_to_node.len()
    }

    pub fn insert(&mut self, node: ArcNodeLink) -> ArcNodeLink{
        let id = node.borrow().get_id();
        if let Some(node) = self.hash_to_node.get(&id) {
            return node.clone()
        }else{
            let mut child_nodes = vec![];
            if let Some(childrens) =  node.borrow_mut().get_children(){
                for children in childrens {
                    let child_node = self.insert(children.clone());
                    child_nodes.push(child_node);
                }
            }

            let new_node: ArcNodeLink = self.create_new_node(&node, child_nodes.as_mut_slice());
            self.hash_to_node.insert(new_node.borrow().get_id(), new_node.clone());
            return new_node
        }
    }

    pub fn get_m(&self) -> u32{
        let mut max = 0;
        for x in &self.hash_to_node {
            let m = x.1.borrow().get_level(0);
            max = m.max(max)
        }
        max
    }

    pub fn matches(&mut self, predicates: &[PredResult]) -> Vec<u64> {
        let mut queues: HashMap<u32, VecDeque<ArcNodeLink>> = HashMap::new();
        let mut matching_exprs = vec![];
        let m = self.get_m();
        for i in (1..m){
            queues.insert(i, VecDeque::new());
        }
        for predicate in predicates {
            if let  Some(ref mut node) = self.hash_to_node.get(&predicate.id){
                if let NodeType::LeafNodeType(ref mut node) = node.borrow_mut().deref_mut() {
                    node.result = predicate.result;
                }
                queues.get_mut(&1).unwrap().push_front(node.clone());
            }
        }

        for x in (1..m) {
            while let Some(node) = queues.get_mut(&x).unwrap().pop_front() {
                let result = node.borrow().evaluate();
                node.borrow_mut().clean();
                if let None = result {
                    continue;
                }

                if let Some(parents) = node.borrow_mut().get_parents(){
                    for parent in parents {

                        match parent.borrow_mut().deref_mut() {
                            NodeType::InnerNodeType(p) => {
                                if p.operands.is_empty() {
                                    let level = p.get_level(1);
                                    let mut queue = queues.get_mut(&level).unwrap();
                                    queue.push_front(parent.clone());
                                }
                                p.operands.push(result);
                            }
                            NodeType::RootNodeType(p) => {
                                if p.operands.is_empty() {
                                    let level = p.get_level(1);
                                    queues.get_mut(&level).unwrap().push_front(parent.clone());
                                }
                                p.operands.push(result);
                            }
                            _ => {}
                        }
                    }
                    if let Some(true) = result{
                        matching_exprs.push(node.borrow().get_id())
                    }
                }
            }
        }
        return matching_exprs;
    }

    fn create_new_node(&mut self, node: &ArcNodeLink, child_nodes: &mut [ArcNodeLink]) -> ArcNodeLink{
        let binding = node.borrow();
        let new_node = binding.deref();
        match new_node {
            NodeType::LeafNodeType(_) => {
                let mut leaf = NodeType::new_leaf(LeafNode::new(new_node.get_id()));
                for node in child_nodes {
                    add_children(&mut leaf, node)
                }
                leaf
            }
            NodeType::InnerNodeType(n) => {
                let mut inner = NodeType::new_inner(InnerNode::new(n.log_operation.clone()));
                for mut node in child_nodes {
                    add_children(&mut inner, &mut node)
                }
                inner
            }
            NodeType::RootNodeType(n) => {
                let mut root = NodeType::new_root(RootNode::new(n.log_operation.clone()));
                for mut node in child_nodes {
                    add_children(&mut root, &mut node)
                }
                root
            }
        }
    }
}

struct EventValue{
    pub name: String,
    pub value: Value
}

struct Event{
    values: Vec<EventValue>
}


struct PredicateStore{
    predicates: HashMap<String, Vec<Box<dyn Predicate>>>
}


impl PredicateStore {

    fn new() -> Self{
        Self{
            predicates: HashMap::new()
        }
    }

    fn add(&mut self, attribute: String, predicate: Box<dyn Predicate>){
        let predicates = self.predicates.entry(attribute).or_default();
        predicates.push(predicate);
    }

    fn evaluate(&self, event: &Event) -> Vec<PredResult> {
        let mut result = vec![];
        for value in &event.values {
            if let Some(predicates) = self.predicates.get(&value.name){
                for predicate in predicates {

                    let predicate_result = PredResult{
                        id: predicate.id(),
                        result: Some(false)
                    };
                    result.push(predicate_result)
                }
            }
        }
        result
    }




}

#[cfg(test)]
mod tests{
    use crate::predicates::Value::Int;
    use super::*;

    #[test]
    fn calculate_level_for_three_nodes(){
        let mut leaf = NodeType::new_leaf(LeafNode::new(1));

        let mut inner = NodeType::new_inner(InnerNode::and());
        add_children(&mut inner, &mut leaf);

        let mut root = NodeType::new_root(RootNode::and());
        add_children(&mut root, &mut inner);

        let c = root.borrow().get_children().unwrap();

        assert_eq!(root.borrow().get_level(0), 3);
    }

    #[test]
    fn calculate_level_for_a_depth_of_four(){
        let mut leaf = NodeType::new_leaf(LeafNode::new(1));

        let mut inner = NodeType::new_inner(InnerNode::and());
        add_children(&mut inner, &mut leaf);

        let mut leaf_two = NodeType::new_leaf(LeafNode::new(2));

        let mut inner_two = NodeType::new_inner(InnerNode::and());
        add_children(&mut inner_two,&mut leaf_two);

        add_children(&mut inner, &mut inner_two);

        let mut root = NodeType::new_root(RootNode::and());
        add_children(&mut root, &mut inner);


        let c = root.borrow().get_children().unwrap();

        assert_eq!(root.borrow().get_level(0), 4);

    }

    #[test]
    fn insert_three_nodes(){
        let mut tree = ATree::new();
        {
            let mut leaf = NodeType::new_leaf(LeafNode::new(1));

            let mut inner = NodeType::new_inner(InnerNode::and());
            add_children(&mut inner, &mut leaf);

            let mut root = NodeType::new_root(RootNode::and());
            add_children(&mut root, &mut inner);

            tree.insert(root.clone());
        }

        assert_eq!(1, tree.len())
    }

    #[test]
    fn insert_two_nodes(){
        let mut tree = ATree::new();
        {
            let mut leaf = NodeType::new_leaf(LeafNode::new(1));
            let mut leaf_two = NodeType::new_leaf(LeafNode::new(2));

            let mut root = NodeType::new_root(RootNode::and());
            add_children(&mut root, &mut leaf);
            add_children(&mut root, &mut leaf_two);

            tree.insert(root.clone());
        }

        assert_eq!(3, tree.len());
        assert_eq!(2, tree.get_m());
    }

    #[test]
    fn insert_two_same_root_nodes(){
        let mut tree = ATree::new();
        {
            let mut leaf = NodeType::new_leaf(LeafNode::new(1));
            let mut leaf_two = NodeType::new_leaf(LeafNode::new(2));

            let mut inner = NodeType::new_inner(InnerNode::and());
            add_children(&mut inner, &mut leaf);
            add_children(&mut inner, &mut leaf_two);

            let mut root = NodeType::new_root(RootNode::and());
            add_children(&mut root,&mut inner);

            tree.insert(root.clone());
        }

        {
            let mut leaf = NodeType::new_leaf(LeafNode::new(1));
            let mut leaf_two = NodeType::new_leaf(LeafNode::new(2));

            let mut inner = NodeType::new_inner(InnerNode::and());
            add_children(&mut inner, &mut leaf);
            add_children(&mut inner, &mut leaf_two);

            let mut root = NodeType::new_root(RootNode::and());
            add_children(&mut root,&mut inner);

            tree.insert(root.clone());
        }

        assert_eq!(3, tree.len());
        assert_eq!(3, tree.get_m());
    }

    #[test]
    fn insert_two_dif_root_nodes(){
        let mut tree = ATree::new();
        {
            let mut leaf = NodeType::new_leaf(LeafNode::new(4));
            let mut leaf_two = NodeType::new_leaf(LeafNode::new(6));

            let mut inner = NodeType::new_inner(InnerNode::and());
            add_children(&mut inner, &mut leaf);
            add_children(&mut inner, &mut leaf_two);

            let mut root = NodeType::new_root(RootNode::and());
            add_children(&mut root,&mut inner);

            tree.insert(root.clone());
        }

        {
            let mut leaf = NodeType::new_leaf(LeafNode::new(8));
            let mut leaf_two = NodeType::new_leaf(LeafNode::new(2));

            let mut inner = NodeType::new_inner(InnerNode::or());
            add_children(&mut inner, &mut leaf);
            add_children(&mut inner, &mut leaf_two);

            let mut root = NodeType::new_root(RootNode::and());
            add_children(&mut root,&mut inner);

            tree.insert(root.clone());
        }

        assert_eq!(6, tree.len());
        assert_eq!(3, tree.get_m());
    }

    #[test]
    fn insert_two_dif_root_and_m_4_nodes(){
        let mut tree = ATree::new();
        {
            let mut leaf_one = NodeType::new_leaf(LeafNode::new(4));
            let mut leaf_two = NodeType::new_leaf(LeafNode::new(6));



            let mut root_inner_1_inner_1 = NodeType::new_inner(InnerNode::and());
            add_children(&mut root_inner_1_inner_1, &mut leaf_one);
            add_children(&mut root_inner_1_inner_1, &mut leaf_two);
            let mut root_inner_1_inner_2 = NodeType::new_inner(InnerNode::or());
            add_children(&mut root_inner_1_inner_2, &mut leaf_one);
            add_children(&mut root_inner_1_inner_2, &mut leaf_two);

            let mut root_inner_2_inner_1 = NodeType::new_inner(InnerNode::and());
            add_children(&mut root_inner_2_inner_1, &mut leaf_one);
            add_children(&mut root_inner_2_inner_1, &mut leaf_two);
            let mut root_inner_2_inner_2 = NodeType::new_inner(InnerNode::and());
            add_children(&mut root_inner_2_inner_2, &mut leaf_one);
            add_children(&mut root_inner_2_inner_2, &mut leaf_two);

            let mut root_inner_1 = NodeType::new_inner(InnerNode::and());
            add_children(&mut root_inner_1, &mut root_inner_1_inner_1);
            add_children(&mut root_inner_1, &mut root_inner_1_inner_2);
            let mut root_inner_2 = NodeType::new_inner(InnerNode::and());
            add_children(&mut root_inner_2, &mut root_inner_2_inner_1);
            add_children(&mut root_inner_2, &mut root_inner_2_inner_2);


            let mut root = NodeType::new_root(RootNode::and());
            add_children(&mut root,&mut root_inner_1);
            add_children(&mut root,&mut root_inner_2);

            tree.insert(root.clone());
        }



        assert_eq!(4, tree.get_m());
    }

    #[test]
    fn test_match(){
        let mut tree = ATree::new();

        let eq = predicates::equal(Int(10));
        let gt = predicates::greater(Int(5));

        {

            let mut leaf = NodeType::new_leaf(LeafNode::new(eq.id()));
            let mut leaf_two = NodeType::new_leaf(LeafNode::new(gt.id()));

            let mut inner = NodeType::new_inner(InnerNode::and());
            add_children(&mut inner, &mut leaf);
            add_children(&mut inner, &mut leaf_two);

            let mut root = NodeType::new_root(RootNode::and());
            add_children(&mut root,&mut inner);

            tree.insert(root.clone());
        }





    }

}
