use std::fs::{metadata, File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use crate::models::expense::Expense;

const FILE_PATH: &str = "expense_data.bin";

pub fn save_expense(expense: &Expense) -> Result<(), String> {
    let mut expenses = load_expenses().map_err(|e| e.to_string())?;

    expenses.push(expense.clone());
    let serialized_data = bincode::serialize(&expenses).map_err(|e| e.to_string())?;

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(FILE_PATH)
        .map_err(|e| e.to_string())?;

    file.write_all(&serialized_data).map_err(|e| e.to_string())?;

    Ok(())
}

pub fn load_expenses() -> Result<Vec<Expense>, String> {
    if Path::new(FILE_PATH).exists() {
        let file_metadata = metadata(FILE_PATH).map_err(|e| e.to_string())?;

        if file_metadata.len() == 0 {
            return Ok(Vec::new());
        }

        let mut file = File::open(FILE_PATH).map_err(|e| e.to_string())?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
        bincode::deserialize(&buffer).map_err(|e| e.to_string())
    } else {
        Ok(Vec::new())
    }
}

pub fn clear_expenses() -> Result<(), String> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILE_PATH)
        .map_err(|e| format!("Erro ao limpar despesas: {}", e))?;

    Ok(())
}

pub fn delete_expense_by_id(id: i64) -> Result<(), String> {
    let mut expenses = load_expenses().map_err(|e| e.to_string())?;

    let original_len = expenses.len();
    expenses.retain(|expense| expense.get_id() != id);

    if expenses.len() == original_len {
        return Err(format!("Nenhuma despesa encontrada com o ID {}", id));
    }

    let serialized_data = bincode::serialize(&expenses).map_err(|e| e.to_string())?;

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(FILE_PATH)
        .map_err(|e| e.to_string())?;

    file.write_all(&serialized_data).map_err(|e| e.to_string())?;

    Ok(())
}

pub fn edit_expense_by_id(id: i64, updated_expense: &Expense) -> Result<&Expense, String> {
    let mut expenses = load_expenses().map_err(|e| e.to_string())?;

    if let Some(index) = expenses.iter().position(|expense| expense.get_id() == id) {
        expenses[index] = updated_expense.clone();

        let serialized_data = bincode::serialize(&expenses).map_err(|e| e.to_string())?;

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(FILE_PATH)
            .map_err(|e| e.to_string())?;

        file.write_all(&serialized_data).map_err(|e| e.to_string())?;

        Ok(updated_expense)
    } else {
        Err(format!("Nenhuma despesa encontrada com o ID {}", id))
    }
}