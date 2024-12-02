use crate::services::services_operations;

pub fn get_expenses() {
    match services_operations::list_expenses() {
        Ok(transactions) => {
            if transactions.len() == 0 {
                println!("Não existe despesas no arquivo!");
                return;
            }

            for (_, t) in transactions.iter().enumerate() {
                t.display_expense();
            }
        }
        Err(e) => println!("Erro ao listar despesas: {}", e),
    }
}