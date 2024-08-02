use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;

pub enum Side {
    Left,
    Right
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }

  pub fn insert(& mut self, val: i32) {
    if val < self.val {
        self.do_insert(val, Side::Left);
    } else {
        self.do_insert(val, Side::Right);
    }
  }

  pub fn do_insert(& mut self, val: i32, side: Side) {
    if let Side::Left = side {
        if self.left.is_none() {
            self.left = Some(Rc::new(RefCell::new(TreeNode::new(val))));            
        } else {      
            self.left
                .as_ref()
                .unwrap()
                .borrow_mut()
                .insert(val);
        }    
         
    } else if let Side::Right = side {
        if self.right.is_none() {
            self.right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        } else {
            self.right
                .as_ref()
                .unwrap()
                .borrow_mut()
                .insert(val);
        }
    }
  }
}

pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
    let mut memo: HashSet<i32> = HashSet::new();
    
    return traverse_tree_with_memo(&root, k, &mut memo);
}

pub fn traverse_tree_with_memo(root: &Option<Rc<RefCell<TreeNode>>>, k: i32, memo: &mut HashSet<i32>) -> bool {
    if root.is_none() {
        return false;
    }

    let usable_root = root
        .as_ref()
        .unwrap()
        .borrow();

    if value_has_target_pair(usable_root.val, k, memo) {
        return true;
    } 
    store_value_if_needed(usable_root.val, memo);

    if traverse_tree_with_memo(&usable_root.left, k, memo) {
        return true;
    }

    if traverse_tree_with_memo(&usable_root.right, k, memo) {
        return true;
    }

    return false;
}

pub fn value_has_target_pair(query_value: i32, target_sum: i32, memo: &HashSet<i32>) -> bool {
    return memo.contains(&(target_sum - query_value));
}

pub fn store_value_if_needed(value: i32, memo: &mut HashSet<i32>) {
    if memo.contains(&value) {
        return;
    }
    memo.insert(value);
}

pub fn build_tree(vec: Vec<i32>) -> TreeNode {
    if vec.len() < 1 {panic!("Only supports non-empty trees");}
    let mut tree = TreeNode::new(vec[0]);

    for value in &vec[1..] {
        tree.insert(*value);
    }

    return tree;
}

fn main() {
    let root = build_tree(vec![5,3,6,2,4,7]);
    println!("My tree: {:?}", root);

    println!("find_target: {}", find_target(Some(Rc::new(RefCell::new(root))), 28));
}
