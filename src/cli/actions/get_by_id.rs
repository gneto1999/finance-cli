use crate::services::services_operations;
use crate::utils;

pub fn get_expense_by_id() {
    println!("\n==========  Buscar despesa pelo ID  ==========");

    print!("Insira um ID: ");
    let id = utils::handle_input();

    let id = id.trim().parse().expect("Por favor, digite um número!");

    match services_operations::list_expense_by_id(id) {
        Ok(expense) => expense.display_expense(),
        Err(e) => println!("Erro ao listar despesa: {}", e),
    }
}