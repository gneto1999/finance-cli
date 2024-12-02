use crate::services::services_operations;

pub fn delete_all_expenses() {
    println!("Excluindo todas as despesas...");
    services_operations::delete_expenses();
    println!("Todas as despesas foram exluídas!")
}