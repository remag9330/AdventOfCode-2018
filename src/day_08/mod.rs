use util::*;

pub fn run_part_1(args: &[String]) {
    run_part_n("1", args, sum_metadata_entries);
}

fn sum_metadata_entries(filename: &String) -> AppResult {
    let tree = read_tree(filename)?;

    let result = tree.sum_metadata();
    println!("Sum of metadatas is {}", result);

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

fn read_usize_from(iter: &mut impl Iterator<Item=Result<usize, std::num::ParseIntError>>) -> Result<usize, AppError> {
    let op = iter.next();
    let item = op.ok_or(AppError::AppError(String::from("Invalid file input")))?;
    let res = item.map_err(|_| AppError::AppError(String::from("Invalid file input")))?;
    Ok(res)
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

    fn root_node(&self) -> Option<&Node> {
        self.root_node.and_then(|index| self.nodes.get(index))
    }

    fn add_node(&mut self, node: Node) -> usize {
        let new_index = self.nodes.len();
        self.nodes.push(node);
        new_index
    }

    fn sum_metadata(&self) -> usize {
        self.nodes.iter().map(|n| n.metadata.iter().sum::<usize>()).sum::<usize>()
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

    fn get_tree() -> Tree {
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        parse_to_tree(&input).unwrap()
    }
}