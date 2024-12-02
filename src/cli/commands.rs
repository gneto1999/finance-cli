use crate::cli::actions::{insert, get_all, get_by_id, delete_all, delete_by_id, update_by_id, expense_resume};

pub fn execute(select:&str) {
    match select {
        "1" => insert::insert_expense(),
        "2" => get_all::get_expenses(),
        "3" => get_by_id::get_expense_by_id(),
        "4" => delete_by_id::delete_expanse_by_id(),
        "5" => delete_all::delete_all_expenses(),
        "6" => update_by_id::update_expense_by_id(),
        "7" => expense_resume::expenses_resume(),
        _ => println!("Insira uma opção válida no terminal")
    }
}