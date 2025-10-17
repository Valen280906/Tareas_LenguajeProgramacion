mod estructuras;
mod commands;

use std::env;
use std::io::{self, Write};
use commands::gestor::GestorTareas;
use commands::{agregar, listar, eliminar, cambiar_estado};
use estructuras::Estado;

fn print_help() {
    println!("Comandos disponibles:");
    println!("- agregar <título> | <descripción>");
    println!("- listar");
    println!("- list <filtro>   (filtros: todo, in-progress, done)");
    println!("- enprogreso <id>");
    println!("- completar <id>");
    println!("- cancelar <id> [razón opcional]");
    println!("- eliminar <id>");
    println!("- eliminar todo");
    println!("- ayuda");
    println!("- salir");
}
 
fn main() {
    let mut gestor = GestorTareas::nuevo();
    let mut args_iter = env::args().skip(1).peekable();

    
    if args_iter.peek().is_none() {


        
        let mut reader = String::new();
        println!("=== SISTEMA DE GESTIÓN DE TAREAS (Rust CLI) ===");
        print_help();
        loop {
            print!("\n> ");
            io::stdout().flush().unwrap();
            reader.clear();
            if io::stdin().read_line(&mut reader).is_err() {
                println!("Error al leer la entrada.");
                continue;
            }
            let input = reader.trim();
            if input.is_empty() {
                continue;
            }
            let parts: Vec<&str> = input.splitn(2, ' ').collect();
            let comando = parts[0].to_lowercase();
            match comando.as_str() {
                "agregar" => {
                    if parts.len() < 2 {
                        println!("Uso: agregar <título> | <descripción>");
                        continue;
                    }
                    let partes: Vec<&str> = parts[1].splitn(2, '|').collect();
                    if partes.len() < 2 {
                        println!("Usa '|' entre título y descripción.");
                        continue;
                    }
                    let titulo = partes[0].trim();
                    let desc = partes[1].trim();
                    if let Err(e) = agregar::agregar(&mut gestor, titulo, desc) {
                        println!("Error: {}", e);
                    }
                }
                "listar" | "list" => {
                    if parts.len() == 2 {
                        listar::listar_por_estado(&gestor, parts[1].trim());
                    } else {
                        listar::listar(&gestor);
                    }
                }
                "enprogreso" => {
                    if parts.len() < 2 {
                        println!("Uso: enprogreso <id>");
                        continue;
                    }
                    let id: i32 = match parts[1].trim().parse() {
                        Ok(v) => v,
                        Err(_) => { println!("ID inválido"); continue; }
                    };
                    if let Err(e) = cambiar_estado::cambiar_estado(&mut gestor, id, Estado::EnProgreso) {
                        println!("Error: {}", e);
                    }
                }
                "completar" => {
                    if parts.len() < 2 {
                        println!("Uso: completar <id>");
                        continue;
                    }
                    let id: i32 = match parts[1].trim().parse() {
                        Ok(v) => v,
                        Err(_) => { println!("ID inválido"); continue; }
                    };
                    if let Err(e) = cambiar_estado::cambiar_estado(&mut gestor, id, Estado::Completada) {
                        println!("Error: {}", e);
                    }
                }
                "cancelar" => {
                    if parts.len() < 2 {
                        println!("Uso: cancelar <id> [razón opcional]");
                        continue;
                    }
                    let args_canc: Vec<&str> = parts[1].trim().splitn(2, ' ').collect();
                    let id: i32 = match args_canc[0].parse() {
                        Ok(v) => v,
                        Err(_) => { println!("ID inválido"); continue; }
                    };
                    let razon = if args_canc.len() == 2 { Some(args_canc[1].trim().to_string()) } else { None };
                    if let Err(e) = cambiar_estado::cambiar_estado(&mut gestor, id, Estado::Cancelada(razon)) {
                        println!("Error: {}", e);
                    }
                }
                "eliminar" => {
                    if parts.len() < 2 {
                        println!("Uso: eliminar <id>  o  eliminar todo");
                        continue;
                    }
                    let arg = parts[1].trim().to_lowercase();
                    if arg == "todo" {
                        print!("¿Seguro que deseas eliminar todas las tareas? (s/n): ");
                        io::stdout().flush().unwrap();
                        let mut confirm = String::new();
                        io::stdin().read_line(&mut confirm).unwrap();
                        if confirm.trim().to_lowercase() == "s" {
                            if let Err(e) = eliminar::eliminar_todas_las_tareas(&mut gestor) {
                                println!("Error: {}", e);
                            }
                        } else {
                            println!("Operación cancelada.");
                        }
                    } else {
                        let id: i32 = match arg.parse() {
                            Ok(v) => v,
                            Err(_) => { println!("El ID debe ser un número entero o 'todo'."); continue; }
                        };
                        if let Err(e) = eliminar::eliminar_tarea(&mut gestor, id) {
                            println!("Error: {}", e);
                        }
                    }
                }
                "ayuda" | "help" => print_help(),
                "salir" => {
                    println!("Guardando cambios y saliendo del gestor de tareas...");
                    if let Err(e) = crate::commands::persistencia::guardar_tareas_json(&gestor.tareas) {
                        println!("Warning: no se pudo guardar: {}", e);
                    }
                    return;
                }
                _ => println!("Comando no reconocido. Escribe 'ayuda' para ver las opciones."),
            }
        }
    } else {
        let args: Vec<String> = env::args().skip(1).collect();

        let comando = args[0].to_lowercase();
        match comando.as_str() {
            "agregar" => {
                if args.len() < 3 {
                    eprintln!("Uso: agregar <título> <descripción>");
                    return;
                }
                let titulo = &args[1];
                let desc = &args[2];
                if let Err(e) = agregar::agregar(&mut gestor, titulo, desc) {
                    eprintln!("Error: {}", e);
                }
            }

            "listar" | "list" => {
                if args.len() == 2 {
                    listar::listar_por_estado(&gestor, &args[1]);
                } else {
                    listar::listar(&gestor);
                }
            }

            "enprogreso" => {
                if args.len() < 2 {
                    eprintln!("Uso: enprogreso <id>");
                    return;
                }
                let id: i32 = args[1].parse().unwrap_or_else(|_| {
                    eprintln!("ID inválido");
                    std::process::exit(1);
                });
                if let Err(e) = cambiar_estado::cambiar_estado(&mut gestor, id, Estado::EnProgreso) {
                    eprintln!("Error: {}", e);
                }
            }

            "completar" => {
                if args.len() < 2 {
                    eprintln!("Uso: completar <id>");
                    return;
                }
                let id: i32 = args[1].parse().unwrap_or_else(|_| {
                    eprintln!("ID inválido");
                    std::process::exit(1);
                });
                if let Err(e) = cambiar_estado::cambiar_estado(&mut gestor, id, Estado::Completada) {
                    eprintln!("Error: {}", e);
                }
            }

            "eliminar" => {
                if args.len() < 2 {
                    eprintln!("Uso: eliminar <id> o eliminar todo");
                    return;
                }
                if args[1].to_lowercase() == "todo" {
                    if let Err(e) = eliminar::eliminar_todas_las_tareas(&mut gestor) {
                        eprintln!("Error: {}", e);
                    }
                } else {
                    let id: i32 = args[1].parse().unwrap_or_else(|_| {
                        eprintln!("ID inválido");
                        std::process::exit(1);
                    });
                    if let Err(e) = eliminar::eliminar_tarea(&mut gestor, id) {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            "ayuda" | "help" => print_help(),

            _ => {
                eprintln!("Comando no reconocido. Escribe 'ayuda' para ver las opciones.");
            }
        }
    }

}
