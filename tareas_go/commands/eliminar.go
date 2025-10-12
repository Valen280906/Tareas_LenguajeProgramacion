package commands

import (
	"errors"
	"fmt"
	"os"
	"tareas_go/estructuras"
)

func (g *GestorTareas) EliminarTarea(id int) error {
	for i, t := range g.tareas {
		if t.ID == id {
			g.tareas = append(g.tareas[:i], g.tareas[i+1:]...)
			g.GuardarTareasJSON()
			return nil
		}
	}
	return errors.New("no se encontró la tarea para eliminar")
}

func (g *GestorTareas) EliminarTodasLasTareas() error {

	if len(g.tareas) == 0 {
		return errors.New("no hay tareas para eliminar")
	}

	fmt.Println("Eliminando todas las tareas...")

	g.tareas = []estructuras.Tarea{}

	err := os.Remove(archivoTareas)
	if err != nil && !os.IsNotExist(err) {
		return fmt.Errorf("error al eliminar archivo JSON: %v", err)
	}

	g.GuardarTareasJSON()
	g.nextID = 1

	fmt.Println("Todas las tareas han sido eliminadas correctamente.")
	return nil
}
