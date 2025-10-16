use chrono::{NaiveDate, Utc};
use colored::*;

use crate::models::{Priority, Task};
use crate::read_string;

const FILE_PATH: &str = "tasks.json";

pub struct TaskManager {
    pub tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: Self::load(),
        }
    }

    pub fn save(&self) {
        println!("Salvando tarefas...");
        let tasks_str = match serde_json::to_string_pretty(&self.tasks) {
            Ok(tasks) => tasks,
            Err(err) => {
                println!("Erro ao serializar tarefas: {}", err);
                return;
            }
        };

        match std::fs::write(FILE_PATH, tasks_str) {
            Ok(_) => {
                println!("Tarefas salvas com sucesso");
            }
            Err(err) => {
                println!("Erro ao salvar tarefas: {}", err);
                return;
            }
        }
    }

    pub fn load() -> Vec<Task> {
        println!("Carregando tarefas...");
        let tasks_str = match std::fs::read_to_string(FILE_PATH) {
            Ok(tasks_str) => tasks_str,
            Err(err) => {
                println!("Erro ao carregar tarefas: {}", err);
                return Vec::new();
            }
        };

        match serde_json::from_str(&tasks_str) {
            Ok(tasks) => tasks,
            Err(err) => {
                println!("Erro ao carregar tarefas: {}", err);
                return Vec::new();
            }
        }
    }

    pub fn add_task(&mut self) {
        let title = read_string("Titulo");
        let description = read_string("Descrição");
        let category = read_string("Categoria");
        let priority_str = read_string("Prioridade (Baixa, Media, Alta)");
        let priority = match priority_str.to_lowercase().as_str() {
            "baixa" => Priority::Low,
            "media" => Priority::Medium,
            "alta" => Priority::High,
            _ => Priority::Medium,
        };
        let date_str = read_string("Data de conclusão (DD-MM-YYYY)");
        let date = match NaiveDate::parse_from_str(&date_str, "%d-%m-%Y") {
            Ok(dt) => dt,
            Err(_) => Utc::now().date_naive(),
        };

        let task = Task::new(title, description, category, date, priority);
        println!("Tarefa {} criada com sucesso", task.title);

        self.tasks.push(task);
        println!("Tarefa adicionada na lista com sucesso");
    }

    pub fn list_tasks(&self) {
        println!("Listando tarefas...");

        if self.tasks.is_empty() {
            println!("Nenhuma tarefa encontrada");
            return;
        }

        for task in &self.tasks {
            println!("{}", task);
        }
    }

    pub fn finish_task(&mut self) {
        if self.tasks.is_empty() {
            println!("Nenhuma tarefa encontrada");
            return;
        }

        for (index, task) in self.tasks.iter().enumerate() {
            println!("{} - {}", index, task);
        }
        let index = read_string("Digite o índice da tarefa").parse::<usize>();
        let task = match index {
            Ok(index) => self.tasks.get_mut(index),
            Err(_) => {
                println!("Opção inválida");
                None
            }
        };

        match task {
            Some(task) => {
                task.finish();
                println!("Tarefa {} finalizada com sucesso", task.title);
            }
            None => println!("Tarefa não encontrada"),
        }
    }

    pub fn delete_task(&mut self) {
        if self.tasks.is_empty() {
            println!("Nenhuma tarefa encontrada");
            return;
        }

        for (index, task) in self.tasks.iter().enumerate() {
            println!("{} - {}", index, task);
        }

        match read_string("Digite o índice da tarefa").parse::<usize>() {
            Ok(index) => {
                if index >= self.tasks.len() {
                    println!("{}", "Índice inválido".to_string().red().bold());
                    return;
                }

                self.tasks.remove(index);
                println!("Tarefa excluída com sucesso");
            }
            Err(_) => {
                println!("Opção inválida");
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Status;

    #[test]
    fn test_task_manager() {
        let mut manager = TaskManager::new();
        manager.tasks.push(Task::new(
            "Teste".to_string(),
            "Teste".to_string(),
            "Teste".to_string(),
            Utc::now().date_naive(),
            Priority::Medium,
        ));
        assert_eq!(manager.tasks[0].status, Status::Pending);
    }
}
