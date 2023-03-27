use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct TreeNode {
    name: String,
    size: u32,
    children: HashMap<String, Rc<RefCell<TreeNode>>>,
    parent: Option<Weak<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(name: String, size: u32) -> TreeNode {
        return TreeNode {
            name,
            size,
            children: HashMap::new(),
            parent: None,
        };
    }
}

fn add_child(parent: &Rc<RefCell<TreeNode>>, child: &Rc<RefCell<TreeNode>>) {
    parent
        .borrow_mut()
        .children
        .insert(child.borrow().name.clone(), Rc::clone(child));
    child.borrow_mut().parent = Some(Weak::clone(&Rc::downgrade(parent)));
}

pub fn solve(input: &Vec<String>) -> String {
    let root = Rc::new(RefCell::new(TreeNode::new("/".to_string(), 0)));
    let mut curr = Rc::clone(&root);

    let mut input_iter = input.into_iter();
    let mut line = input_iter.nth(1);

    // Build filesystem tree
    while let Some(command) = line {
        if command.starts_with("$ ls") {
            line = input_iter.next();
            while let Some(output) = line {
                if output.starts_with("$") == true {
                    break;
                }
                let tokens = output.split(" ").collect::<Vec<_>>();
                let size: u32;
                if tokens[0] == "dir" {
                    size = 0;
                } else {
                    size = tokens[0].parse().unwrap();
                }
                let child = Rc::new(RefCell::new(TreeNode::new(tokens[1].to_string(), size)));
                add_child(&curr, &child);
                line = input_iter.next();
            }
        } else {
            let tokens = command.split(" ").collect::<Vec<_>>();
            if tokens[2] == ".." {
                let parent = curr.borrow().parent.clone().unwrap();
                curr = Rc::clone(&parent.upgrade().unwrap());
            } else {
                let dir = tokens[2].to_string();
                let new_node = Rc::clone(curr.borrow().children.get(&dir).unwrap());
                curr = new_node;
            }
            line = input_iter.next();
        }
    }

    // Compute directory sizes
    compute_directory_sizes(&root);

    // Compute size
    let mut size: u32 = 0;
    get_size(&root, &mut size);
    size.to_string()

    // println!("{:?}", root);
    // "".to_string()
}

fn compute_directory_sizes(node: &Rc<RefCell<TreeNode>>) {
    let mut size = 0;
    for (_, child) in &node.borrow().children {
        if child.borrow().size == 0 {
            compute_directory_sizes(&child);
        }
        size += child.borrow().size;
    }
    node.borrow_mut().size = size;
}


fn get_size(node: &Rc<RefCell<TreeNode>>, size: &mut u32) {
    for (_, child) in &node.borrow().children {
        if child.borrow().children.len() != 0 {
            get_size(child, size);
            if child.borrow().size < 100000 {
                *size += child.borrow().size;
            }
        }
    }
}
