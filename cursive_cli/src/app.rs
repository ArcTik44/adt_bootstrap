use array_sort::sort::sort;
use binary_merge::binary_merge::binary_tree_merge;
use binary_tree::binary_tree::binary_tree_create;
use cursive::{
    views::{Button, Dialog, LinearLayout, TextView},
    Cursive,
};
use huffman_en::huffman_encoding::huffman_encoding;

pub fn start_huffman(s: &mut Cursive, string: &str) {
    s.pop_layer();

    let result = huffman_encoding(string);
    /*Label layout */
    let labels = LinearLayout::vertical()
        .child(TextView::new("VÝSTUP:"))
        .child(TextView::new("VSTUP:"))
        .child(TextView::new("KÓD-ZNAK:"))
        .child(TextView::new("FREKVENCE:"));

    /*Results layout */
    let results = LinearLayout::vertical()
        .child(TextView::new(&result[0]))
        .child(TextView::new(string.to_string()))
        .child(TextView::new(&result[2]))
        .child(TextView::new(&result[1]));

    /*Button layout */
    let buttons = LinearLayout::horizontal().child(Button::new("Exit", |s: &mut Cursive|{
        s.pop_layer();
    }));

    /*Rendering in window */
    s.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(labels)
                .child(results)
                .child(buttons),
        ) 
        .title("VÝSTUPY"),
    );
}

pub fn start_heap_sort(s: &mut Cursive, to_vec: &str) {
    s.pop_layer();

    let result = sort(to_vec);
    /*Label layout */
    let labels = LinearLayout::vertical()
        .child(TextView::new("VSTUP:"))
        .child(TextView::new("VÝSTUP:"));

    /*Button layout */
    let buttons = LinearLayout::horizontal().child(Button::new("Exit", |s|{
        s.pop_layer();
    }));

    /*Results layout */
    let results = LinearLayout::vertical()
        .child(TextView::new(to_vec.to_string()))
        .child(TextView::new(result));

    /*Rendering in window */
    s.add_layer(Dialog::around(
        LinearLayout::horizontal()
            .child(labels)
            .child(results)
            .child(buttons),
    ));
}

pub fn start_binary_tree(s: &mut Cursive, to_vec: &str) {
    s.pop_layer();

    let traverses = binary_tree_create(to_vec);

    /*Label layout */
    let labels = LinearLayout::vertical()
        .child(TextView::new("VSTUP:"))
        .child(TextView::new("IN ORDER:"))
        .child(TextView::new("PRE ORDER:"))
        .child(TextView::new("POST ORDER:"));

    /*Button layout */
    let buttons = LinearLayout::horizontal().child(Button::new("Back", |s|{
        s.pop_layer();
    }));

    /*Result layout */
    let results = LinearLayout::vertical()
        .child(TextView::new(to_vec))
        .child(TextView::new(&traverses[0]))
        .child(TextView::new(&traverses[1]))
        .child(TextView::new(&traverses[2]));

    /*Rendering in window */
    s.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(labels)
                .child(results)
                .child(buttons),
        )
        .title("VÝSTUPY"),
    );
}

pub fn start_binary_merge(s: &mut Cursive, to_vec1: &str, to_vec2: &str) {
    s.pop_layer();

    let result = binary_tree_merge(to_vec1, to_vec2);

    /*Label layout */
    let labels = LinearLayout::vertical()
        .child(TextView::new("VSTUP1:"))
        .child(TextView::new("VSTUP2:"))
        .child(TextView::new("SLOUČENÉ ():"));

    /*Buttons layout */
    let buttons = LinearLayout::horizontal().child(Button::new("Back", |s|{
        s.pop_layer();
    }));
    
    /*Results layout */
    let results = LinearLayout::vertical()
        .child(TextView::new(to_vec1))
        .child(TextView::new(to_vec2))
        .child(TextView::new(result));

    /*Rendering in window */
    s.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(labels)
                .child(results)
                .child(buttons),
        )
        .title("VÝSTUPY"),
    );
}
