use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    size: u32,
    children: Vec<String>,
}

pub fn solve(input: &Vec<String>) -> String {
    let mut input_iter = input.iter();
    let mut path: Vec<&str> = vec![];
    let mut nodes: HashMap<String, Node> = HashMap::new();

    // Root Node
    nodes.insert(
        "".to_string(),
        Node {
            size: 0,
            children: vec![],
        },
    );

    // Tree Construction
    let mut line = input_iter.nth(1);
    while let Some(command) = line {
        if command.starts_with("$ cd") {
            let dir = command.split(" ").nth(2).unwrap();
            if dir == ".." {
                path.pop();
            } else {
                path.push(dir);
            }
            line = input_iter.next();
        } else {
            line = input_iter.next();
            while let Some(output) = line {
                if output.starts_with("$") {
                    break;
                }

                let tokens = output.split(" ").collect::<Vec<_>>();

                let node: Node;
                if tokens[0] == "dir" {
                    node = Node {
                        size: 0,
                        children: vec![],
                    };
                } else {
                    node = Node {
                        size: tokens[0].parse().unwrap(),
                        children: vec![],
                    };
                }

                let parent_path = path.join("/");
                path.push(tokens[1]);
                let child_path = path.join("/");
                nodes
                    .get_mut(&parent_path)
                    .unwrap()
                    .children
                    .push(child_path.clone());
                nodes.insert(child_path, node);
                path.pop();

                line = input_iter.next();
            }
        }
    }

    // Calculate sizes
    let mut sizes: HashMap<String, u32> = HashMap::new();
    compute_dir_sizes(&"".to_string(), &nodes, &mut sizes);

    sizes
        .values()
        .filter(|size| **size < 100000)
        .sum::<u32>()
        .to_string()
}

fn compute_dir_sizes(
    path: &String,
    nodes: &HashMap<String, Node>,
    sizes: &mut HashMap<String, u32>,
) {
    let mut size: u32 = 0;
    for child in nodes.get(path).unwrap().children.iter() {
        let child_node = nodes.get(child).unwrap();
        if child_node.size == 0 && child_node.children.len() != 0 {
            compute_dir_sizes(child, nodes, sizes);
            size += sizes.get(child).unwrap();
        } else {
            size += child_node.size;
        }
    }
    sizes.insert(path.clone(), size);
}
