use serde::{Serialize, Deserialize};
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Estado {
    Pendiente,
    EnProgreso,
    Completada,
    Cancelada(Option<String>), 
}

impl fmt::Display for Estado {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Estado::Pendiente => write!(f, "Pendiente"),
            Estado::EnProgreso => write!(f, "En progreso"),
            Estado::Completada => write!(f, "Completada"),
            Estado::Cancelada(Some(razon)) => write!(f, "Cancelada ({})", razon),
            Estado::Cancelada(None) => write!(f, "Cancelada"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tarea {
    pub id: i32,
    pub titulo: String,
    pub descripcion: String,
    pub estado: Estado,
    pub fecha: u64,
}

impl Tarea {
    pub fn nueva(id: i32, titulo: &str, descripcion: &str) -> Self {
        let ahora = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Tarea {
            id,
            titulo: titulo.to_string(),
            descripcion: descripcion.to_string(),
            estado: Estado::Pendiente,
            fecha: ahora,
        }
    }

    pub fn cambiar_estado(&mut self, nuevo: Estado) {
        self.estado = nuevo;
    }

    pub fn mostrar(&self) {
        println!("[{}] {} ({})", self.id, self.titulo, self.estado);
        println!("Fecha de creación: {}", self.formatear_fecha());
        println!("Descripción: {}", self.descripcion);
        println!("------------------------------------------------------");
    }

    fn formatear_fecha(&self) -> String {
        let tiempo = std::time::UNIX_EPOCH + std::time::Duration::from_secs(self.fecha);
        let datetime: chrono::DateTime<chrono::Local> = tiempo.into();
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}