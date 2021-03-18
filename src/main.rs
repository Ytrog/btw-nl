use dialoguer::{Confirm, Editor, Select, MultiSelect, console::Term, Input};

fn main() {
    let term = Term::stdout();
    term.write_line("BTW berekenaar").unwrap();
}
