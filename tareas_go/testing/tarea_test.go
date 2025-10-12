package testing

import (
	"os"
	"tareas_go/commands"
	"tareas_go/estructuras"
	"testing"
)

func limpiarJSON() {
	os.Remove("tareas.json")
}

func TestAgregarTarea(t *testing.T) {
	limpiarJSON()
	gestor := commands.NuevoGestor()
	tarea := gestor.AgregarTarea("Probar función", "Verificar creación de tarea")

	if tarea.Titulo != "Probar función" {
		t.Errorf("Título incorrecto: %s", tarea.Titulo)
	}
	if tarea.Estado != estructuras.Pendiente {
		t.Errorf("Estado inicial incorrecto: %v", tarea.Estado)
	}
}

func TestCambiarEstado(t *testing.T) {
	limpiarJSON()
	gestor := commands.NuevoGestor()
	tarea := gestor.AgregarTarea("Prueba", "Cambio de estado")

	err := gestor.CambiarEstado(tarea.ID, estructuras.Completada)
	if err != nil {
		t.Errorf("Error al cambiar estado: %v", err)
	}

	for _, tareaGuardada := range gestor.Tareas() {
		if tareaGuardada.ID == tarea.ID && tareaGuardada.Estado != estructuras.Completada {
			t.Errorf("El estado no cambió correctamente")
		}
	}
}

func TestEliminarTarea(t *testing.T) {
	limpiarJSON()
	gestor := commands.NuevoGestor()
	t1 := gestor.AgregarTarea("Tarea 1", "Desc 1")
	t2 := gestor.AgregarTarea("Tarea 2", "Desc 2")

	err := gestor.EliminarTarea(t2.ID)
	if err != nil {
		t.Errorf("Error al eliminar: %v", err)
	}

	for _, tarea := range gestor.Tareas() {
		if tarea.ID == t2.ID {
			t.Errorf("La tarea %d no fue eliminada correctamente", t2.ID)
		}
	}

	encontrada := false
	for _, tarea := range gestor.Tareas() {
		if tarea.ID == t1.ID {
			encontrada = true
		}
	}
	if !encontrada {
		t.Errorf("La tarea 1 fue eliminada por error")
	}
}

func TestCambiarEstadoInvalido(t *testing.T) {
	tarea := estructuras.Tarea{ID: 1, Estado: estructuras.Pendiente}
	err := tarea.CambiarEstado(99)
	if err == nil {
		t.Errorf("Se esperaba error por estado inválido")
	}
}

func TestEliminarTodasLasTareas(t *testing.T) {
	limpiarJSON()
	gestor := commands.NuevoGestor()
	gestor.AgregarTarea("Tarea 1", "Desc 1")
	gestor.AgregarTarea("Tarea 2", "Desc 2")

	err := gestor.EliminarTodasLasTareas()
	if err != nil {
		t.Errorf("Error al eliminar todas las tareas: %v", err)
	}

	if len(gestor.Tareas()) != 0 {
		t.Errorf("No se eliminaron todas las tareas correctamente")
	}
}
