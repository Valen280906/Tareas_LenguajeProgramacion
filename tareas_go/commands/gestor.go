package commands

import "tareas_go/estructuras"

type GestorTareas struct {
	tareas []estructuras.Tarea
	nextID int
}

func NuevoGestor() *GestorTareas {
	g := &GestorTareas{
		tareas: make([]estructuras.Tarea, 0),
		nextID: 1,
	}

	g.CargarTareasJSON()

	return g
}

func (g *GestorTareas) Tareas() []estructuras.Tarea {
	return g.tareas
}
