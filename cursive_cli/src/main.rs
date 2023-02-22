mod app;
use app::{start_binary_merge, start_binary_tree, start_heap_sort, start_huffman};
use cursive::view::{Nameable, Resizable};
use cursive::views::{Button, Dialog, EditView, LinearLayout, TextView};
use cursive::Cursive;

fn main() {
    let mut app = cursive::default();

    let heaps = LinearLayout::vertical()
        .child(TextView::new("HALDY").center())
        .child(Button::new("HEAP SORT", enter_heap_sort))
        .child(Button::new("HUFFMANOVO KÓDOVÁNÍ", enter_huffman));

    let trees = LinearLayout::vertical()
        .child(TextView::new("BINÁRNÍ STROM").center())
        .child(Button::new(
            "TRAVERZOVÁNÍ BINÁRNÍHO STROMU",
            enter_binary_order,
        ))
        .child(Button::new("SLOUČENÍ 2 BINÁRNÍ STROMŮ", enter_binary_merge))
        .child(Button::new("Exit", Cursive::quit));

    app.add_layer(
        Dialog::around(LinearLayout::horizontal().child(heaps).child(trees))
            .title("ABSTRAKTNÍ DATOVÉ TYPY"),
    );
    app.set_window_title("ADT BOOTSTRAP");
    app.run();
}

fn enter_heap_sort(s: &mut Cursive) {
    s.add_layer(
        Dialog::around(EditView::new().with_name("input").fixed_width(30))
            .title("HEAP SORT")
            .button("Back", |s: &mut Cursive| {s.pop_layer();})
            .button("Start", |s: &mut Cursive| {
                let arr = s
                    .call_on_name("input", |v: &mut EditView| v.get_content())
                    .unwrap();
                start_heap_sort(s, arr.as_str());
            }),
    );
}

fn enter_huffman(s: &mut Cursive) {
    s.add_layer(
        Dialog::around(EditView::new().with_name("input").fixed_width(30))
            .title("HUFFMANOVO KÓDOVÁNÍ")
            .button("Back", |s|{s.pop_layer();})
            .button("Start", |s| {
                let test = s
                    .call_on_name("input", |v: &mut EditView| v.get_content())
                    .unwrap();
                start_huffman(s, test.as_str());
            }),
    );
}

fn enter_binary_order(s: &mut Cursive) {
    s.add_layer(
        Dialog::around(EditView::new().with_name("input").fixed_width(30))
            .title("TRAVERZOVÁNÍ BINÁRNÍHO STROMU")
            .button("Back", |s|{s.pop_layer();})
            .button("Start", |s| {
                let input = s
                    .call_on_name("input", |v: &mut EditView| v.get_content())
                    .unwrap();
                start_binary_tree(s, input.as_str())
            }),
    );
}

fn enter_binary_merge(s: &mut Cursive) {
    let input_f1 = EditView::new().with_name("input_f1").fixed_width(30);

    let input_f2 = EditView::new().with_name("input_f2").fixed_width(30);

    let input_fields = LinearLayout::vertical().child(input_f1).child(input_f2);

    s.add_layer(
        Dialog::around(input_fields)
            .title("SLOUČENÍ 2 BINÁRNÍCH STROMŮ")
            .button("Back", |s|{s.pop_layer();})
            .button("Start", |s| {
                let input_val1 = s
                    .call_on_name("input_f1", |v: &mut EditView| v.get_content())
                    .unwrap();
                let input_val2 = s
                    .call_on_name("input_f2", |v: &mut EditView| v.get_content())
                    .unwrap();
                start_binary_merge(s, input_val1.as_str(), input_val2.as_str())
            }),
    );
}
