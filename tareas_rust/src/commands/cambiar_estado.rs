use crate::commands::persistencia;
use crate::commands::gestor::GestorTareas;
use crate::estructuras::Estado;

pub fn cambiar_estado(g: &mut GestorTareas, id: i32, nuevo: Estado) -> Result<(), String> {
    if let Some(t) = g.tareas.iter_mut().find(|t| t.id == id) {
        t.cambiar_estado(nuevo);
        persistencia::guardar_tareas_json(&g.tareas)?;
        println!("Estado de la tarea {} cambiado correctamente.", id);
        return Ok(());
    }
    Err(format!("Tarea {} no encontrada.", id))
}
