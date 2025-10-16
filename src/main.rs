mod io_utils;
mod models;
mod task_manager;

use std::sync::{Arc, Mutex};

use colored::*;

use io_utils::{clear_terminal, read_string};
use task_manager::TaskManager;

fn print_yellow(text: &str) {
    println!("{}", text.to_string().yellow().bold().underline());
}

fn main() {
    let _ = clear_terminal();
    let manager = Arc::new(Mutex::new(TaskManager::new()));
    let manager_clone = Arc::clone(&manager);

    ctrlc::set_handler(move || {
        println!("Sinal detectado, encerrando...");
        if let Ok(mgr) = manager_clone.lock() {
            mgr.save();
        }
        std::process::exit(0);
    })
    .expect("Erro ao registrar handler Ctrl+C");

    let mut mgr = manager.lock().unwrap();

    loop {
        println!(
            "{}",
            "Gerenciador de Tarefas"
                .to_string()
                .purple()
                .bold()
                .underline()
        );
        println!("{}", "======================".to_string().purple().bold());
        println!("");

        print_yellow("1) Adicionar tarefa");
        print_yellow("2) Listar tarefas");
        print_yellow("3) Marcar como finalizada");
        print_yellow("4) Deletar tarefa");
        print_yellow("5) Sair");
        println!("");

        let option = read_string("Escolha uma opção");

        match option.trim() {
            "1" => {
                let _ = clear_terminal();
                mgr.add_task();
            }
            "2" => {
                let _ = clear_terminal();
                mgr.list_tasks();
            }
            "3" => {
                let _ = clear_terminal();
                mgr.finish_task();
            }
            "4" => {
                let _ = clear_terminal();
                mgr.delete_task();
            }
            "5" => {
                mgr.save();
                println!("Obrigado por usar o gerenciador de tarefas");
                break;
            }
            _ => {
                mgr.save();
                println!("Opção inválida.");
                break;
            }
        }
    }
}
