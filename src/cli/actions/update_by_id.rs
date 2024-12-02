use crate::models::expense::Expense;
use crate::services::services_operations;
use crate::utils;

pub fn update_expense_by_id() {
    println!("\n==========  Edidtar despesa pelo ID  ==========");

    print!("Insira um ID: ");
    let id = utils::handle_input();

    let id = id.trim().parse().expect("Por favor, digite um número!");

    println!("\nDespesa alvo para mudanças:");

    match services_operations::list_expense_by_id(id) {
        Ok(expense) => expense.display_expense(),
        Err(e) => println!("Erro ao listar despesa6: {}", e),
    }
    println!("Dados para serem modificados:");
    println!("Os campos que não serão alterados precisam ser preechidos com os dados da versão original!");

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

    let update_expense = Expense::new(title, description, amount, date, category);

    match services_operations::update_expense_by_id(id, &update_expense) {
        Ok(expense) => expense.display_expense(),
        Err(e) => println!("Erro ao listar despesa atualizada: {}", e),
    };
}
