use std::io;
use crate::cli::commands;
pub fn menu () {
    loop {
        println!("======================  Gestor de Despesas  ========================");
        println!("1) Adicionar despesa");
        println!("2) Listar todas as despesas");
        println!("3) Listar despesa pelo ID");
        println!("4) Excluir despesa pelo ID");
        println!("5) Excluir todas as despesas");
        println!("6) Editar despesa pelo ID");
        println!("7) Listar total das despesas, maior valor, menor valor");
        println!("8) SAIR");
        println!("====================================================================");

        let mut select = String::new();

        io::stdin().read_line(&mut select)
            .expect("Falha ao ler entrada");

        let cleaned_select = select.trim();

        if cleaned_select == "8" {
            println!("Saiu do programa!");
            return
        }

        commands::execute(cleaned_select);
    }
}