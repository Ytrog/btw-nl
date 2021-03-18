use dialoguer::{console::Style, console::Term, Confirm, Input, Select};

enum Action {
    BtwBrutoHoog,
    BtwBrutoLaag,
    BtwNettoHoog,
    BtwNettoLaag,
    Stop,
}

/// pause console
fn pause() {
    println!("Druk op een toets om verder te gaan");
    Term::stdout().read_key().unwrap();
}

/// clear the screen
fn clear() {
    Term::stdout().clear_screen().unwrap();
}

fn ask_bruto(percentage: u8) {
    let input: f64 = Input::new().with_prompt("Bedrag").interact_text().unwrap();
    println!("{}", calc_bruto(input, percentage));
    pause();
}

fn calc_bruto(value: f64, percentage: u8) -> f64 {
    let p = f64::from(percentage); // u8 to prevent negatives
    value / (100.0 + p) * p
}

fn stop() -> bool {
    Confirm::new()
        .with_prompt("Stoppen?")
        .interact()
        .unwrap_or(true)
}

fn get_action() -> Action {
    clear();
    let action = Select::new()
        .with_prompt(
            "Wat wilt u doen (gebruik pijltjestoetsen om te selecteren en vervolgens enter)",
        )
        .item("Btw hoog berekenen vanuit bruto bedrag")
        .item("Btw laag berekenen vanuit bruto bedrag")
        .item("Btw hoog berekenen vanuit netto bedrag")
        .item("Btw laag berekenen vanuit netto bedrag")
        .item(Style::new().red().bright().apply_to("Stoppen"))
        .interact()
        .unwrap();

    match action {
        0 => Action::BtwBrutoHoog,
        1 => Action::BtwBrutoLaag,
        2 => Action::BtwNettoHoog,
        3 => Action::BtwNettoLaag,
        4 => Action::Stop,
        _ => Action::Stop,
    }
}

fn main() {
    let term = Term::stdout();
    term.write_line("BTW berekenaar").unwrap();
    let _warn = Style::new().red();
    term.set_title("BTW berekenaar");

    loop {
        match get_action() {
            Action::BtwBrutoHoog => ask_bruto(21),
            Action::BtwBrutoLaag => ask_bruto(9),
            Action::Stop => {
                if stop() {
                    break;
                }
            }
            _ => unreachable!(),
        }
    }
}
