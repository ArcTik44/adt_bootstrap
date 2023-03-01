pub fn binary_tree_merge(tree1: &str, tree2: &str) -> String {
    let split1 = tree1.split(";");
    let split2 = tree2.split(";");

    let mut arr1: Vec<i32> = Vec::new();
    let mut arr2: Vec<i32> = Vec::new();

    for num in split1 {
        if let Ok(n) = num.parse::<i32>() {
            arr1.push(n);
        }
    }

    for num in split2 {
        if let Ok(n) = num.parse::<i32>() {
            arr2.push(n);
        }
    }
    arr1.sort();
    arr2.sort();

    let bin1 = build_tree(arr1);
    let bin2 = build_tree(arr2);

    let bin_merged = merge_trees(bin1, bin2);

    let merged_str = prettier_output(&bin_merged);
    return merged_str;
}
struct Node {
    item: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

/*Funkce pro sestavení stromu z pole */
fn build_tree(vals: Vec<i32>) -> Option<Box<Node>> {
    if vals.is_empty() {
        return None;
    }
    let mid = vals.len() / 2;
    let item = vals[mid];

    let left = build_tree(vals[0..mid].to_vec());
    let right = build_tree(vals[mid + 1..].to_vec());
    return Some(Box::new(Node { item, left, right }));
}

/*Funkce pro sloučení binární stromů*/
fn merge_trees(tree1: Option<Box<Node>>, tree2: Option<Box<Node>>) -> Option<Box<Node>> {
    /*Rekurzivní průchod */
    match (tree1, tree2) {
        /*Pokud oba stromy obsahují uzel na stejné pozici, jejich hodnota se sečte do nového uzlu na stejné pozici v */
        (Some(node1), Some(node2)) => {
            return Some(Box::new(Node {
                item: node1.item + node2.item,
                left: merge_trees(node1.left, node2.left),
                right: merge_trees(node1.right, node2.right),
            }))
        }
        ,
        /*Pokud se uzel s touto pozicí nachází pouze v jednom ze stromů, přidá se jako nový uzel do nového stromu na stejnou pozici*/
        (None, Some(node2)) => return Some(node2),
        (Some(node1), None) => return Some(node1),

        (None, None) => return None,
    }
}

/*Funkce pro vypsání */
fn prettier_output(tree: &Option<Box<Node>>) -> String {
    let mut result = String::new();
    match tree {
        Some(tree) => {
            result.push_str((tree.item.to_string()).as_str());
            result.push(';');
            result.push_str(&prettier_output(&tree.left));
            result.push_str(&prettier_output(&tree.right));
        }
        None => {}
    }
    return result;
}
