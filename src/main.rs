use dialoguer::{Confirm, Editor, Select, MultiSelect, console::Term, console::Style, Input};

fn stop() -> bool{
    Confirm::new().with_prompt("Stoppen?").interact().unwrap_or(true)
}

fn get_action() -> usize {
    let action = Select::new()
    .with_prompt("Wat wilt u doen (gebruik pijltjestoetsen om te selecteren en vervolgens enter)")
    .item("Btw hoog berekenen vanuit bruto bedrag")
    .item("Btw laag berekenen vanuit bruto bedrag")
    .item("Btw hoog berekenen vanuit netto bedrag")
    .item("Btw laag berekenen vanuit netto bedrag")
    .item(Style::new().red().bright().apply_to("Stoppen"))
    .interact().unwrap();

    action
}

fn main() {
    let term = Term::stdout();
    term.write_line("BTW berekenaar").unwrap();
    let _warn = Style::new().red();
    term.set_title("BTW berekenaar");

    loop {
    match get_action() {
        a if a < 4 => println!("{}", a),
        4 => if stop() {
            break;
        },
        _ => unreachable!()
    }
    }
}
