pub fn binary_tree_create(seq: &str) -> Vec<String> {
    let split = seq.split(";");
    let mut traverses = Vec::new();
    let mut arr: Vec<i32> = Vec::new();

    for num in split {
        let insert: i32 = num.parse().unwrap();
        arr.push(insert);
    }
    arr.sort();

    let tree = build_tree(arr);

    traverses.push(in_order_traversal(&tree));
    traverses.push(post_order_traversal(&tree));
    traverses.push(pre_order_traversal(&tree));

    return traverses;
}

#[derive(Debug)]
struct Node {
    item: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

/*Strom můžeme sestavit pouze v případě */
fn build_tree(mut vals: Vec<i32>) -> Option<Box<Node>> {
    vals.sort();
    if vals.is_empty() {
        return None;
    }
    let mid = vals.len() / 2;
    let item = vals[mid];

    let left = build_tree(vals[0..mid].to_vec());
    let right = build_tree(vals[mid + 1..].to_vec());
    return Some(Box::new(Node { item, left, right }));
}

/*začíná v nejmenším, končí v největším */

fn in_order_traversal(root: &Option<Box<Node>>) -> String {
    let mut tree: String = String::new();
    if let Some(node) = root {
        tree.push_str(&in_order_traversal(&node.left));
        tree.push_str((node.item.to_string()).as_str());
        tree.push(';');
        tree.push_str(&in_order_traversal(&node.right));
    }
    return tree;
}
/*Funguje na podobném principu jako DFS (začíná v rootu, končí ve spodním pravém uzlu */
fn post_order_traversal(root: &Option<Box<Node>>) -> String {
    let mut tree: String = String::new();
    if let Some(node) = root {
        tree.push_str(&post_order_traversal(&node.left));
        tree.push_str(&post_order_traversal(&node.right));
        tree.push_str((node.item.to_string()).as_str());
        tree.push(';');
    }
    return tree;
}
/*Začíná v nejmenším potomku končí v rootu */
fn pre_order_traversal(root: &Option<Box<Node>>) -> String {
    let mut tree: String = String::new();
    if let Some(node) = root {
        tree.push_str((node.item.to_string()).as_str());
        tree.push(';');
        tree.push_str(&pre_order_traversal(&node.left));
        tree.push_str(&pre_order_traversal(&node.right));
    }
    return tree;
}
