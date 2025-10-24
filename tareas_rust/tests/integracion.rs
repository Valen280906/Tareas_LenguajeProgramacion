use tempfile::NamedTempFile;
use std::path::PathBuf;

use tareas_rust::commands::{agregar, cambiar_estado, eliminar};
use tareas_rust::commands::gestor::GestorTareas;
use tareas_rust::commands::persistencia;
use tareas_rust::estructuras::Estado;

fn preparar_entorno_prueba(nombre: &str) -> PathBuf {
    let tmp = NamedTempFile::new().expect("No se pudo crear archivo temporal");
    let ruta = tmp.into_temp_path().to_path_buf();

    println!("🧪 Archivo temporal creado para '{}': {:?}", nombre, ruta);
    ruta
}

#[test]
fn test_agregar_tarea() {
    let archivo = preparar_entorno_prueba("agregar");
    let mut gestor = GestorTareas::nuevo_con_archivo(&archivo);

    let resultado = agregar::agregar(&mut gestor, "Probar test", "Verificar funcionamiento");
    assert!(resultado.is_ok(), "Fallo al agregar tarea");
    assert_eq!(gestor.tareas.len(), 1);

    let tarea = &gestor.tareas[0];
    assert_eq!(tarea.titulo, "Probar test");
    assert!(matches!(tarea.estado, Estado::Pendiente));

    persistencia::guardar_tareas_json_con_ruta(&gestor.tareas, &archivo).unwrap();
}

#[test]
fn test_cambiar_estado() {
    let archivo = preparar_entorno_prueba("cambiar_estado");
    let mut gestor = GestorTareas::nuevo_con_archivo(&archivo);

    agregar::agregar(&mut gestor, "Tarea 1", "Cambiar estado").unwrap();
    let id = gestor.tareas[0].id;

    let resultado = cambiar_estado::cambiar_estado(&mut gestor, id, Estado::Completada);
    assert!(resultado.is_ok());
    assert!(matches!(gestor.tareas[0].estado, Estado::Completada));
}

#[test]
fn test_eliminar_tarea() {
    let archivo = preparar_entorno_prueba("eliminar");
    let mut gestor = GestorTareas::nuevo_con_archivo(&archivo);

    agregar::agregar(&mut gestor, "Tarea a eliminar", "Probando borrado").unwrap();
    let id = gestor.tareas[0].id;

    let resultado = eliminar::eliminar_tarea(&mut gestor, id);
    assert!(resultado.is_ok());
    assert_eq!(gestor.tareas.len(), 0);
}

#[test]
fn test_eliminar_todo() {
    let archivo = preparar_entorno_prueba("eliminar_todo");
    let mut gestor = GestorTareas::nuevo_con_archivo(&archivo);

    agregar::agregar(&mut gestor, "Tarea 1", "Primera").unwrap();
    agregar::agregar(&mut gestor, "Tarea 2", "Segunda").unwrap();

    let resultado = eliminar::eliminar_todas_las_tareas(&mut gestor);
    assert!(resultado.is_ok());
    assert!(gestor.tareas.is_empty());
}
