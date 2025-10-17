use crate::estructuras::Tarea;
use crate::commands::persistencia;

#[derive(Debug)]
pub struct GestorTareas {
    pub tareas: Vec<Tarea>,
    pub next_id: i32,
}

impl GestorTareas {
    pub fn nuevo() -> Self {
        match persistencia::cargar_tareas_json() {
            Ok(tareas) => {
                let next_id = if tareas.is_empty() {
                    1
                } else {
                    tareas.iter().map(|t| t.id).max().unwrap_or(0) + 1
                };
                println!("✅ Tareas cargadas desde JSON. Total: {}", tareas.len());
                GestorTareas { tareas, next_id }
            }
            Err(e) => {
                eprintln!("No se pudo cargar el archivo JSON: {}. Se inicia vacío.", e);
                GestorTareas {
                    tareas: Vec::new(),
                    next_id: 1,
                }
            }
        }
    }

    pub fn tareas(&self) -> &Vec<Tarea> {
        &self.tareas
    }
}
