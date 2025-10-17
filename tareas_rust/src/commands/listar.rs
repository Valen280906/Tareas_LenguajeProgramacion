use crate::commands::gestor::GestorTareas;
use crate::estructuras::Estado;

pub fn listar(g: &GestorTareas) {
    if g.tareas.is_empty() {
        println!("No hay tareas registradas.");
        return;
    }
    for t in &g.tareas {
        t.mostrar();
    }
}

pub fn listar_por_estado(g: &GestorTareas, estado: &str) {
    let mut encontradas = 0;
    for t in &g.tareas {
        let coincide = match estado {
            "todo" | "pendiente" => matches!(t.estado, Estado::Pendiente),
            "in-progress" | "enprogreso" => matches!(t.estado, Estado::EnProgreso),
            "done" | "completada" => matches!(t.estado, Estado::Completada),
            _ => true,
        };
        if coincide {
            t.mostrar();
            encontradas += 1;
        }
    }
    if encontradas == 0 {
        println!("No se encontraron tareas con el filtro '{}'.", estado);
    }
}
