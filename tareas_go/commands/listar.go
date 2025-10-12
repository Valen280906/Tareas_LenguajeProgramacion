package commands

import "fmt"

func (g *GestorTareas) ListarTareas() {
	if len(g.tareas) == 0 {
		fmt.Println("No hay tareas registradas.")
		return
	}
	for _, t := range g.tareas {
		t.Mostrar()
	}
}
