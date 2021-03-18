use dialoguer::{Confirm, Editor, Select, MultiSelect, console::Term, console::Style, Input};

enum Action {
    BtwBrutoHoog,
    BtwBrutoLaag,
    BtwNettoHoog,
    BtwNettoLaag,
    Stop
}

fn stop() -> bool{
    Confirm::new().with_prompt("Stoppen?").interact().unwrap_or(true)
}

fn get_action() -> Action {
    let action = Select::new()
    .with_prompt("Wat wilt u doen (gebruik pijltjestoetsen om te selecteren en vervolgens enter)")
    .item("Btw hoog berekenen vanuit bruto bedrag")
    .item("Btw laag berekenen vanuit bruto bedrag")
    .item("Btw hoog berekenen vanuit netto bedrag")
    .item("Btw laag berekenen vanuit netto bedrag")
    .item(Style::new().red().bright().apply_to("Stoppen"))
    .interact().unwrap();

    match action {
        0 => Action::BtwBrutoHoog,
        1 => Action::BtwBrutoLaag,
        2 => Action::BtwNettoHoog,
        3 => Action::BtwNettoLaag,
        4 => Action::Stop,
        _ => Action::Stop
    }
}

fn main() {
    let term = Term::stdout();
    term.write_line("BTW berekenaar").unwrap();
    let _warn = Style::new().red();
    term.set_title("BTW berekenaar");

    loop {
    match get_action() {
        Action::BtwBrutoHoog => continue,
        Action::Stop => if stop() {
            break;
        },
        _ => unreachable!()
    }
    }
}
