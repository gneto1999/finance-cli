use crate::models::expense::Expense;
use crate::repository::persistence;

pub fn add_expense(expense: &Expense) {
    persistence::save_expense(expense).expect("TODO: panic message");
}

pub fn list_expenses() -> Result<Vec<Expense>, String> {
    persistence::load_expenses()
}

pub fn list_expense_by_id(id: i64) -> Result<Expense, String> {
    let expense = persistence::load_expenses()?;
    expense.into_iter().find(|e| e.get_id() == id).ok_or(format!("No expense with id {}", id))
}

pub fn delete_expenses() {
    persistence::clear_expenses().expect("Falha ao limpar despesas");
}

pub fn delete_expense_by_id(id: i64) {
    persistence::delete_expense_by_id(id).expect("Falha ao deletar despesa com id {id}");
}

pub fn update_expense_by_id(id: i64, expense: &Expense) -> Result<&Expense, String> {
    persistence::edit_expense_by_id(id, expense)
}

pub fn calculate_expense_statistics() -> (f64, Option<Expense>, Option<Expense>) {
    let expenses = persistence::load_expenses().expect("Falha ao carregar a lista com as despesas!");

    let total: f64 = expenses.iter().map(|expense| expense.get_amount()).sum();

    let max_expense = expenses.iter()
        .cloned()
        .max_by(|a, b| a
            .get_amount()
            .partial_cmp(&b.get_amount())
            .unwrap_or(std::cmp::Ordering::Equal));

    let min_expense = expenses.iter()
        .cloned()
        .min_by(|a, b| a
            .get_amount()
            .partial_cmp(&b.get_amount())
            .unwrap_or(std::cmp::Ordering::Equal));

    (total, max_expense, min_expense)
}