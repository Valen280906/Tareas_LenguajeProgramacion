use crate::commands::persistencia;
use crate::commands::gestor::GestorTareas;

pub fn eliminar_tarea(g: &mut GestorTareas, id: i32) -> Result<(), String> {
    if let Some(pos) = g.tareas.iter().position(|t| t.id == id) {
        g.tareas.remove(pos);
        persistencia::guardar_tareas_json(&g.tareas)?;
        println!("Tarea {} eliminada correctamente.", id);
        return Ok(());
    }
    Err(format!("No se encontró la tarea con id {}.", id))
}

pub fn eliminar_todas_las_tareas(g: &mut GestorTareas) -> Result<(), String> {
    if g.tareas.is_empty() {
        return Err("No hay tareas para eliminar.".to_string());
    }
    g.tareas.clear();
    
    match std::fs::remove_file("tareas.json") {
        Ok(_) => (),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
        Err(e) => return Err(format!("Error al eliminar archivo JSON: {}", e)),
    }

    persistencia::guardar_tareas_json(&g.tareas)?;
    g.next_id = 1;
    println!("Todas las tareas han sido eliminadas correctamente.");
    Ok(())
}
