use std::fmt;
use chrono::NaiveDate;
use idgenerator::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Category {
    Alimentacao,
    Transporte,
    Lazer,
    Saude,
    Educacao,
    Cobrancas,
    Outros
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Category::Alimentacao => write!(f, "Alimentação"),
            Category::Transporte => write!(f, "Transporte"),
            Category::Lazer => write!(f, "Lazer"),
            Category::Saude => write!(f, "Saúde"),
            Category::Educacao => write!(f, "Educação"),
            Category::Cobrancas => write!(f, "Cobranças"),
            Category::Outros => write!(f, "Outros"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Expense {
    id: i64,
    title: String,
    description: String,
    amount: f64,
    date: NaiveDate,
    category: Category
}

pub fn generate_id() -> Result<i64, String> {
    let options = IdGeneratorOptions::new().worker_id(1).worker_id_bit_len(6);
    IdInstance::init(options).map_err(|err|(format!("Error initializing ID generator: {}", err)))?;
    Ok(IdInstance::next_id())
}

impl Expense {
    pub fn new(title: String, description: String, amount: String, date: String, category: String) -> Expense {
        let id = match generate_id() {
            Ok(id) => id,
            Err(e) => {
                panic!("{e}")
            },
        };

        let amount: f64 = amount.trim().parse()
            .expect("Por favor, digite um número!");

        let category = match category.trim().to_lowercase().as_str() {
            "alimentação" => Category::Alimentacao,
            "transporte" => Category::Transporte,
            "lazer" => Category::Lazer,
            "saúde" => Category::Saude,
            "educação" => Category::Educacao,
            "cobranças" => Category::Cobrancas,
            "outros" => Category::Outros,
            _ => {
                panic!("Categoria invalida {category}")
            }
        };

        let date = match NaiveDate::parse_from_str(&date, "%d/%m/%Y") {
            Ok(date) => date,
            Err(_) => panic!("Date invalid {date}")
        };

        Expense {
            id,
            title,
            description,
            amount,
            date,
            category
        }
    }

    pub fn get_id(&self) -> i64 {
        self.id
    }

    pub fn get_amount(&self) -> f64 {
        self.amount
    }

    pub fn display_expense(&self) {
        print!("\n[ID: {}]:\nTítulo: {}, \nDescrição: {}, \nValor: {}, \nData: {}, \nCategoria: {}\n\n",
               self.id.to_string(),
               self.title,
               self.description,
               self.amount, self.date,
               self.category);
    }
}