use dialoguer::{Confirm, Editor, Select, MultiSelect, console::Term, Input};

fn get_action() -> usize {
    let action = Select::new()
    .with_prompt("Wat wilt u doen (gebruik pijltjestoetsen om te selecteren en vervolgens enter)")
    .item("Btw hoog berekenen vanuit bruto bedrag")
    .item("Btw laag berekenen vanuit bruto bedrag")
    .item("Btw hoog berekenen vanuit netto bedrag")
    .item("Btw laag berekenen vanuit netto bedrag")
    .item("Stoppen")
    .interact().unwrap();

    action
}

fn main() {
    let term = Term::stdout();
    term.write_line("BTW berekenaar").unwrap();
    let warn = console::Style::new().red();

    match get_action() {
        a if a < 4 => println!("{}", a),
        4 => println!("{}", warn.apply_to("Stoppen")),
        _ => unreachable!()
    }
}
