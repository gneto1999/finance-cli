use crate::utils;
use crate::models::expense::Expense;
use crate::services::services_operations;

pub fn insert_expense() {
    println!("\n==========  Inserção de Despesa  ==========");
    print!("Insira o título: ");
    let title = utils::handle_input();
    print!("Insira a descrição: ");
    let description = utils::handle_input();
    print!("Insira o valor da despesa: ");
    let amount = utils::handle_input();
    print!("Insira a data da despesa (dd/MM/yyyy): ");
    let date = utils::handle_input();
    print!("Insira a categoria: ");
    let category = utils::handle_input();

    let expense = Expense::new(title, description, amount, date, category);

    services_operations::add_expense(&expense);
    utils::clear_terminal();
}