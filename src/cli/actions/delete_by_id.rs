use crate::utils;
use crate::services::services_operations;

pub fn delete_expanse_by_id() {
    println!("\n==========  Deletar despesa pelo ID  ==========");

    print!("Insira um ID: ");
    let id = utils::handle_input();

    let id = id.trim().parse().expect("Por favor, digite um número!");

    services_operations::delete_expense_by_id(id);
}