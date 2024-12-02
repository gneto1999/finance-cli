#[cfg(test)]
mod tests {
    use crate::models::expense::{Expense, Category};
    use crate::services::services_operations::{
        add_expense, calculate_expense_statistics, delete_expense_by_id, delete_expenses,
        list_expense_by_id, list_expenses, update_expense_by_id,
    };

    #[test]
    fn test_add_expense() {
        delete_expenses();

        let expense = Expense::new("1".to_string(), "Teste".to_string(), "10.0".to_string(), "02/12/2024".to_string(), Category::Transporte.to_string());
        add_expense(&expense);

        let loaded_expenses = list_expenses().unwrap();
        assert_eq!(loaded_expenses.len(), 1);
        assert_eq!(loaded_expenses.get(0), Some(&expense));  // Comparando com Option<&Expense>
    }

    #[test]
    fn test_list_expenses() {
        delete_expenses();

        let expense1 = Expense::new("1".to_string(), "Teste 1".to_string(), "10.0".to_string(), "02/12/2024".to_string(), Category::Transporte.to_string());
        let expense2 = Expense::new("2".to_string(), "Teste 2".to_string(), "20.0".to_string(), "03/12/2024".to_string(), Category::Lazer.to_string());

        add_expense(&expense1);
        add_expense(&expense2);

        let expenses = list_expenses().unwrap();
        assert_eq!(expenses.len(), 2);
        assert!(expenses.contains(&expense1));
        assert!(expenses.contains(&expense2));
    }

    #[test]
    fn test_list_expense_by_id() {
        delete_expenses();

        let expense = Expense::new("1".to_string(), "Teste".to_string(), "10.0".to_string(), "02/12/2024".to_string(), Category::Transporte.to_string());
        add_expense(&expense);

        let retrieved_expense = list_expense_by_id(expense.get_id()).unwrap();
        assert_eq!(retrieved_expense, expense);

        let result = list_expense_by_id(2i64);
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_expenses() {
        let expense = Expense::new("1".to_string(), "Teste".to_string(), "10.0".to_string(), "02/12/2024".to_string(), Category::Transporte.to_string());
        add_expense(&expense);

        delete_expenses();
        let expenses = list_expenses().unwrap();
        assert!(expenses.is_empty());
    }

    #[test]
    fn test_delete_expense_by_id() {
        delete_expenses();

        let expense1 = Expense::new("1".to_string(), "Teste 1".to_string(), "10.0".to_string(), "02/12/2024".to_string(), Category::Transporte.to_string());
        let expense2 = Expense::new("2".to_string(), "Teste 2".to_string(), "20.0".to_string(), "03/12/2024".to_string(), Category::Lazer.to_string());

        add_expense(&expense1);
        add_expense(&expense2);

        delete_expense_by_id(expense1.get_id());

        let expenses = list_expenses().unwrap();
        assert_eq!(expenses.len(), 1);
        assert_eq!(expenses[0], expense2);

        let result = list_expense_by_id(expense1.get_id());
        assert!(result.is_err());
    }

    #[test]
    fn test_update_expense_by_id() {
        delete_expenses();

        let expense = Expense::new("1".to_string(), "Teste 1".to_string(), "10.0".to_string(), "02/12/2024".to_string(), Category::Transporte.to_string());
        add_expense(&expense);

        let updated_expense = Expense::new("1".to_string(), "TESTE 1".to_string(), "20.0".to_string(), "02/12/2024".to_string(), Category::Alimentacao.to_string());
        let result = update_expense_by_id(expense.get_id(), &updated_expense);
        let id_update = result.clone().unwrap().get_id();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), &updated_expense);

        let retrieved_expense = list_expense_by_id(id_update).unwrap();
        assert_eq!(retrieved_expense, updated_expense);

        let result = update_expense_by_id(2i64, &updated_expense);
        assert!(result.is_err());
    }

    #[test]
    fn test_calculate_expense_statistics() {
        delete_expenses();

        let expense1 = Expense::new("1".to_string(), "Teste 1".to_string(), "20.0".to_string(), "02/12/2024".to_string(), Category::Transporte.to_string());
        let expense2 = Expense::new("2".to_string(), "Teste 2".to_string(), "10.0".to_string(), "03/12/2024".to_string(), Category::Lazer.to_string());
        let expense3 = Expense::new("3".to_string(), "Teste 3".to_string(), "50.0".to_string(), "04/12/2024".to_string(), Category::Saude.to_string());

        add_expense(&expense1);
        add_expense(&expense2);
        add_expense(&expense3);

        let (total, max_expense, min_expense) = calculate_expense_statistics();

        assert_eq!(total, 80.0);
        assert_eq!(max_expense, Some(expense3));
        assert_eq!(min_expense, Some(expense2));
    }
}
