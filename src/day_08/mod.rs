use crate::util::*;

pub fn run_part_1(args: &[String]) {
    run_part_n("1", args, sum_metadata_entries);
}

pub fn run_part_2(args: &[String]) {
    run_part_n("2", args, calculate_checksum);
}

fn sum_metadata_entries(filename: &String) -> AppResult {
    let tree = read_tree(filename)?;
    let result = tree.sum_metadata();

    println!("Sum of metadatas is {}", result);

    Ok(())
}

fn calculate_checksum(filename: &String) -> AppResult {
    let tree = read_tree(filename)?;
    let result = tree.checksum();

    println!("Tree checksum is {}", result);

    Ok(())
}

fn read_tree(filename: &String) -> Result<Tree, AppError> {
    let input = read_file_input(filename)?;
    Ok(parse_to_tree(&input)?)
}

fn parse_to_tree(input: &str) -> Result<Tree, AppError> {
    let mut tree = Tree::new();
    let mut iter = input.split(' ').map(|s| s.parse::<usize>());

    let root = read_node(&mut tree, &mut iter)?;
    tree.root_node = Some(root);

    Ok(tree)
}

fn read_node(tree: &mut Tree, iter: &mut impl Iterator<Item=Result<usize, std::num::ParseIntError>>) -> Result<usize, AppError> {
    let mut new_node = Node::new();
    let child_count = read_usize_from(iter)?;
    
    let metadata_count = read_usize_from(iter)?;
    
    for _ in 0..child_count {
        new_node.children.push(read_node(tree, iter)?);
    }

    for _ in 0..metadata_count {
        new_node.metadata.push(read_usize_from(iter)?)
    }

    Ok(tree.add_node(new_node))
}

fn read_usize_from<T>(iter: &mut T) -> Result<usize, AppError> 
        where T: Iterator<Item=Result<usize, std::num::ParseIntError>> {
    match iter.next() {
        Some(Ok(result)) => Ok(result),
        None => Err(AppError::AppError(String::from("Unexpected EOF when parsing file"))),
        Some(Err(err)) => Err(AppError::AppError(String::from(format!("Unable to convert string to number. {}", err)))),
    }
}

struct Tree {
    nodes: Vec<Node>,
    root_node: Option<usize>,
}

impl Tree {
    fn new() -> Tree {
        Tree {
            nodes: Vec::new(),
            root_node: None,
        }
    }

    fn add_node(&mut self, node: Node) -> usize {
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    fn sum_metadata(&self) -> usize {
        self.nodes.iter()
            .map(|n| n.metadata.iter().sum::<usize>())
            .sum::<usize>()
    }

    fn checksum(&self) -> usize {
        self.root_node
            .map(|n| self.node_checksum(n))
            .unwrap_or(0)
    }

    fn node_checksum(&self, node_id: usize) -> usize {
        let node = self.nodes.get(node_id).unwrap();
        if node.children.len() == 0 {
            node.metadata.iter().sum::<usize>()
        } else {
            let mut result = 0;

            for index in node.metadata.iter() {
                if let Some(child) = node.children.get(*index - 1) {
                    result += self.node_checksum(*child);
                }
            }

            result
        }
    }
}

struct Node {
    children: Vec<usize>,
    metadata: Vec<usize>,
}

impl Node {
    fn new() -> Node {
        Node {
            children: Vec::new(),
            metadata: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_metadata() {
        let tree = get_tree();
        assert_eq!(138, tree.sum_metadata());
    }

    #[test]
    fn test_node_checksum() {
        let tree = get_tree();
        assert_eq!(66, tree.checksum());
    }

    fn get_tree() -> Tree {
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        parse_to_tree(&input).unwrap()
    }
}
