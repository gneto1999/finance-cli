use crate::services::services_operations;

pub fn expenses_resume() {
    let (total, max_expense, min_expense) = services_operations::calculate_expense_statistics();

    println!("Total: R${:.2}\n", total);

    match max_expense {
        Some(expense) => {
            println!("Maior despesa:");
            expense.display_expense();
        },
        None => println!("Não há maior despesa."),
    }

    match min_expense {
        Some(expense) => {
            println!("Menor despesa:");
            expense.display_expense();
        },
        None => println!("Não há menor despesa."),
    }
}