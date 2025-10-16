mod estructuras;
mod commands;

use commands::gestor; // Importa el módulo gestor

fn main() {
    println!("=== SISTEMA DE GESTIÓN DE TAREAS (Rust CLI) ===");
    gestor::saludar();
}
