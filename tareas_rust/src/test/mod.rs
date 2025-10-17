// ======================================
// 🧪 PRUEBAS UNITARIAS DEL GESTOR DE TAREAS
// ======================================

#[cfg(test)]
mod tests {
    // Importamos lo que necesitamos
    use super::*;
    use crate::estructuras::Tarea;

    #[test]
    fn test_agregar_tarea() {
        let mut gestor = GestorTareas::new();
        gestor.agregar_tarea("Probar test", "Verificar funcionamiento");

        assert_eq!(gestor.tareas.len(), 1);
        assert_eq!(gestor.tareas[0].titulo, "Probar test");
        assert_eq!(gestor.tareas[0].descripcion, "Verificar funcionamiento");
        assert_eq!(gestor.tareas[0].estado, "Pendiente");
    }

    #[test]
    fn test_cambiar_estado() {
        let mut gestor = GestorTareas::new();
        gestor.agregar_tarea("Tarea 1", "Cambiar estado");

        let id = gestor.tareas[0].id;
        let resultado = gestor.cambiar_estado(id, "Completada");

        assert!(resultado.is_ok());
        assert_eq!(gestor.tareas[0].estado, "Completada");
    }

    #[test]
    fn test_eliminar_tarea() {
        let mut gestor = GestorTareas::new();
        gestor.agregar_tarea("Tarea a eliminar", "Probando borrado");
        let id = gestor.tareas[0].id;

        let resultado = gestor.eliminar_tarea(id);
        assert!(resultado.is_ok());
        assert_eq!(gestor.tareas.len(), 0);
    }

    #[test]
    fn test_buscar_tarea_inexistente() {
        let gestor = GestorTareas::new();
        let resultado = gestor.buscar_tarea(999);
        assert!(resultado.is_none());
    }
}
