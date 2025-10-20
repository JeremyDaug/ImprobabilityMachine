use std::io::stdout;

use crossterm::{style::Print, terminal, ExecutableCommand};

use crate::common_state::{self, CommonState};

pub fn machine_screen(common_state: &mut CommonState) {
    stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    let mut msg = String::new();
    loop {
        stdout().execute(Print("\tImprobability Machine Screen\n")).unwrap();
        stdout().execute(Print(
            format!("Money: ${}\tEntropy: {} b\n\n", common_state.money, common_state.entropy)
        )).unwrap();
        stdout().execute(
            Print("Commands: R -> Return to Game Select | Enter Number to increase Entropy Capacity\n")
        ).unwrap();
        stdout().execute(
            Print(format!("Entropy Max: {}\tMachine Level: {}\n", 
            common_state.machine.entropy_cap(), 
            common_state.machine.level))
        ).unwrap();
        stdout().execute(Print(
            format!("{}\n\n", msg)
        )).unwrap();
        let mut buff = String::new();
        buff = buff.trim_end().to_string();
        if let Ok(invest) = buff.parse::<f64>() {
            if invest + 1.0 > common_state.money {
                // Must have at least 1 dollar left to continue betting.
                msg = String::from("Cannot invest that much, must have at least $1 left.");
            } else {
                let upgrade = (invest / 2.0).floor();
                let expense = upgrade * 2.0;
                msg = String::from(format!("Gained {} levels of entropy.", upgrade));
                common_state.money -= expense;
                common_state.machine.level += upgrade;
            }
        }
    }
}