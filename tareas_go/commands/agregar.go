package commands

import (
	"fmt"
	"tareas_go/estructuras"
	"time"
)

func (g *GestorTareas) AgregarTarea(titulo, descripcion string) estructuras.Tarea {
	nueva := estructuras.Tarea{
		ID:          g.nextID,
		Titulo:      titulo,
		Descripcion: descripcion,
		Estado:      estructuras.Pendiente,
		Fecha:       time.Now(),
	}

	g.tareas = append(g.tareas, nueva)
	g.nextID++

	g.GuardarTareasJSON()

	fmt.Printf("Tarea agregada: [%d] %s\n", nueva.ID, nueva.Titulo)
	return nueva
}
