package commands

import (
	"fmt"
	"tareas_go/estructuras"
)

func (g *GestorTareas) CambiarEstado(id int, nuevo estructuras.Estado) error {
	for i := range g.tareas {
		if g.tareas[i].ID == id {
			g.tareas[i].Estado = nuevo
			g.GuardarTareasJSON()
			fmt.Printf("Estado de la tarea %d cambiado a %s.\n", id, estructuras.EstadoToString(nuevo))
			return nil
		}
	}
	return fmt.Errorf("tarea no encontrada")
}
