use std::env;
use std::fs;

use tareas_rust::commands::{agregar, cambiar_estado, eliminar};
use tareas_rust::commands::gestor::GestorTareas;
use tareas_rust::estructuras::{Estado, Tarea as _}; 

fn preparar_entorno_prueba(nombre: &str) {
    let archivo_test = format!("test_{}.json", nombre);
    std::env::set_var("ARCHIVO_TAREAS", archivo_test.clone());
    let _ = std::fs::remove_file(archivo_test);
}



#[test]
fn test_agregar_tarea() {
    preparar_entorno_prueba("agregar");

    let mut gestor = GestorTareas::nuevo();
    let resultado = agregar::agregar(&mut gestor, "Probar test", "Verificar funcionamiento");

    assert!(resultado.is_ok(), "Fallo al agregar tarea");
    assert_eq!(gestor.tareas.len(), 1, "Debe haber exactamente una tarea agregada");

    let tarea = &gestor.tareas[0];
    assert_eq!(tarea.titulo, "Probar test");
    assert_eq!(tarea.descripcion, "Verificar funcionamiento");
    assert!(matches!(tarea.estado, Estado::Pendiente));
}

#[test]
fn test_cambiar_estado() {
    preparar_entorno_prueba("cambiar_estado");

    let mut gestor = GestorTareas::nuevo();
    agregar::agregar(&mut gestor, "Tarea 1", "Cambiar estado").unwrap();

    let id = gestor.tareas[0].id;
    let resultado = cambiar_estado::cambiar_estado(&mut gestor, id, Estado::Completada);

    assert!(resultado.is_ok(), "Fallo al cambiar el estado de la tarea");
    assert!(matches!(gestor.tareas[0].estado, Estado::Completada));
}

#[test]
fn test_eliminar_tarea() {
    preparar_entorno_prueba("eliminar");

    let mut gestor = GestorTareas::nuevo();
    agregar::agregar(&mut gestor, "Tarea a eliminar", "Probando borrado").unwrap();

    let id = gestor.tareas[0].id;
    let resultado = eliminar::eliminar_tarea(&mut gestor, id);

    assert!(resultado.is_ok(), "Fallo al eliminar la tarea");
    assert_eq!(gestor.tareas.len(), 0, "Después de eliminar debe quedar vacío");
}

#[test]
fn test_eliminar_todo() {
    preparar_entorno_prueba("eliminar_todo");

    let mut gestor = GestorTareas::nuevo();
    agregar::agregar(&mut gestor, "Tarea 1", "Primera").unwrap();
    agregar::agregar(&mut gestor, "Tarea 2", "Segunda").unwrap();

    let resultado = eliminar::eliminar_todas_las_tareas(&mut gestor);
    assert!(resultado.is_ok(), "Fallo al eliminar todas las tareas");
    assert!(gestor.tareas.is_empty(), "Después de eliminar todo, no deben quedar tareas");
}
