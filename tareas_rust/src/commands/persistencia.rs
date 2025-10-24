use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::path::PathBuf;

use crate::estructuras::Tarea;

const ARCHIVO_TAREAS: &str = "tareas.json";

pub fn guardar_tareas_json(tareas: &Vec<Tarea>) -> Result<(), String> {
    match serde_json::to_string_pretty(tareas) {
        Ok(json) => {
            match File::create(ARCHIVO_TAREAS) {
                Ok(mut archivo) => {
                    if let Err(e) = archivo.write_all(json.as_bytes()) {
                        return Err(format!("Error al escribir el archivo: {}", e));
                    }
                    Ok(())
                }
                Err(e) => Err(format!("No se pudo crear el archivo: {}", e)),
            }
        }
        Err(e) => Err(format!("Error al convertir las tareas a JSON: {}", e)),
    }
}

pub fn cargar_tareas_json() -> Result<Vec<Tarea>, String> {
    if !Path::new(ARCHIVO_TAREAS).exists() {
        return Err("Archivo JSON no encontrado.".to_string());
    }

    let mut archivo = match OpenOptions::new().read(true).open(ARCHIVO_TAREAS) {
        Ok(f) => f,
        Err(e) => return Err(format!("Error al abrir el archivo: {}", e)),
    };

    let mut contenido = String::new();
    if let Err(e) = archivo.read_to_string(&mut contenido) {
        return Err(format!("Error al leer el archivo: {}", e));
    }

    if contenido.trim().is_empty() {
        return Ok(Vec::new());
    }

    match serde_json::from_str::<Vec<Tarea>>(&contenido) {
        Ok(tareas) => Ok(tareas),
        Err(e) => Err(format!("Error al parsear JSON: {}", e)),
    }
}


pub fn guardar_tareas_json_con_ruta(tareas: &Vec<Tarea>, archivo: &PathBuf) -> Result<(), String> {
    match serde_json::to_string_pretty(tareas) {
        Ok(json) => {
            match File::create(archivo) {
                Ok(mut f) => f.write_all(json.as_bytes()).map_err(|e| e.to_string()),
                Err(e) => Err(format!("Error al crear archivo {}: {}", archivo.display(), e)),
            }
        }
        Err(e) => Err(format!("Error al serializar JSON: {}", e)),
    }
}

pub fn cargar_tareas_json_con_ruta(archivo: &PathBuf) -> Result<Vec<Tarea>, String> {
    if !archivo.exists() {
        return Err("Archivo no encontrado".to_string());
    }

    let mut contenido = String::new();
    let mut f = OpenOptions::new().read(true).open(archivo).map_err(|e| e.to_string())?;
    f.read_to_string(&mut contenido).map_err(|e| e.to_string())?;

    if contenido.trim().is_empty() {
        return Ok(Vec::new());
    }

    serde_json::from_str(&contenido).map_err(|e| e.to_string())
}