use std::collections::HashMap;
struct Node {
    ch: Option<char>,
    freq: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

pub fn huffman_encoding(to_encode: &str) -> Vec<String> {
    /*Hashmapa pro ukládání frekvencí jednotlivých charakterů */
    let mut results: Vec<String> = Vec::new();
    let test: HashMap<char, i32> = frequency(to_encode);
    let mut freq: String = String::new();
    let mut codes: String = String::new();
    freq.push_str("{");

    for (item_key, item_value) in &test {
        freq.push(*item_key);
        freq.push_str(":");
        freq.push_str(&item_value.to_string());
        freq.push_str(" ;");
    }
    freq.push_str("}");

    /*Hashmapa pro ukládání binárního kódu*/
    let mut h: HashMap<char, String> = HashMap::new();

    /*Vytvoříme pole Nodů pro každý char v hashmapě */
    let mut p: Vec<Box<Node>> = test
        .iter()
        .map(|x| new_box(new_node(*(x.1), Some(*(x.0)))))
        .collect();
    /*Vytvoření Huffmanova stromu */
    while p.len() > 1 {
        p.sort_by(|a, b| (&(b.freq)).cmp(&(a.freq)));
        let a = p.pop().unwrap();
        let b = p.pop().unwrap();
        let mut c = new_box(new_node(a.freq + b.freq, None));
        c.left = Some(a);
        c.right = Some(b);
        p.push(c);
    }
    let root = p.pop().unwrap();
    generate_codes(&root, &mut h, "".to_string());
    codes.push_str("{");
    for (item_key, item_value) in &h {
        codes.push(*item_key);
        codes.push_str(":");
        codes.push_str(&item_value.to_string());
        codes.push_str(" ;");
    }
    codes.push_str("}");

    results.push(encode_string(to_encode, &h));
    results.push(freq);
    results.push(codes);

    return results;
}

/*Funkce pro zjištění frekvence jednotlivých charakterů v řetězci */
pub fn frequency(s: &str) -> HashMap<char, i32> {
    let mut h = HashMap::new();
    /*cyklus pro procházení řetězce znak po znaku */
    for chr in s.chars() {
        /*Pokud se klíč(znak) již je v Hashmapě nachází, tak se jeho hodnota(frekvence) zvýší o 1*/
        let count = h.entry(chr).or_insert(0);
        *count += 1;
    }
    return h;
}

fn new_node(freq: i32, ch: Option<char>) -> Node {
    Node {
        freq: freq,
        ch: ch,
        left: None,
        right: None,
    }
}

fn new_box(n: Node) -> Box<Node> {
    Box::new(n)
}

/*Generování binárního kódu pro node*/
fn generate_codes(p: &Box<Node>, h: &mut HashMap<char, String>, s: String) {
    if let Some(ch) = p.ch {
        h.insert(ch, s);
    } else {
        if let Some(ref l) = p.left {
            generate_codes(l, h, s.clone() + "0");
        }
        if let Some(ref r) = p.right {
            generate_codes(r, h, s.clone() + "1");
        }
    }
}

fn encode_string(s: &str, h: &HashMap<char, String>) -> String {
    let mut r = "".to_string();
    let mut t: Option<&String>;

    for ch in s.chars() {
        t = h.get(&ch);
        r.push_str(t.unwrap());
    }
    return r;
}