package commands

import (
	"encoding/json"
	"fmt"
	"os"
	"tareas_go/estructuras"
)

const archivoTareas = "tareas.json"

func (g *GestorTareas) GuardarTareasJSON() {
	data, err := json.MarshalIndent(g.tareas, "", "  ")
	if err != nil {
		fmt.Println("Error al convertir tareas a JSON:", err)
		return
	}

	err = os.WriteFile(archivoTareas, data, 0644)
	if err != nil {
		fmt.Println("Error al guardar tareas en archivo:", err)
	}
}

func (g *GestorTareas) CargarTareasJSON() {
	data, err := os.ReadFile(archivoTareas)
	if err != nil {
		fmt.Println("No se encontró archivo JSON, iniciando vacío.")
		return
	}

	var tareas []estructuras.Tarea
	err = json.Unmarshal(data, &tareas)
	if err != nil {
		fmt.Println("Error al leer JSON:", err)
		return
	}

	g.tareas = tareas
	if len(tareas) > 0 {
		g.nextID = tareas[len(tareas)-1].ID + 1
	}
}
