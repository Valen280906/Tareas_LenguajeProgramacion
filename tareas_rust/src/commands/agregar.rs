use crate::commands::persistencia;
use crate::commands::gestor::GestorTareas;
use crate::estructuras::Tarea;

pub fn agregar(g: &mut GestorTareas, titulo: &str, descripcion: &str) -> Result<(), String> {
    let id = g.next_id;
    let nueva = Tarea::nueva(id, titulo, descripcion);
    g.tareas.push(nueva);
    g.next_id += 1;

    persistencia::guardar_tareas_json(&g.tareas)?;
    println!("Tarea agregada: [{}] {}", id, g.tareas.last().unwrap().titulo);
    Ok(())
}
