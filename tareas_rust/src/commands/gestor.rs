use crate::estructuras::Tarea;
use crate::commands::persistencia;
use std::path::PathBuf;

#[derive(Debug)]
pub struct GestorTareas {
    pub tareas: Vec<Tarea>,
    pub next_id: i32,
     pub archivo: PathBuf,
}

impl GestorTareas {
    pub fn nuevo() -> Self {
        Self::nuevo_con_archivo("tareas.json")
    }

    pub fn nuevo_con_archivo<P: Into<PathBuf>>(archivo: P) -> Self {
        let ruta = archivo.into();
        match persistencia::cargar_tareas_json_con_ruta(&ruta) {
            Ok(tareas) => {
                let next_id = if tareas.is_empty() {
                    1
                } else {
                    tareas.iter().map(|t| t.id).max().unwrap_or(0) + 1
                };
                println!("Tareas cargadas desde {:?}. Total: {}", ruta, tareas.len());
                GestorTareas { tareas, next_id, archivo: ruta }
            }
            Err(_) => GestorTareas { tareas: Vec::new(), next_id: 1, archivo: ruta },
        }
    }
}