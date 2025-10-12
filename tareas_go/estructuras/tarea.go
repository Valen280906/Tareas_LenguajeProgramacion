package estructuras

import (
	"errors"
	"fmt"
	"time"
)

type Estado int

const (
	Pendiente Estado = iota
	EnProgreso
	Completada
	Cancelada
)

type Tarea struct {
	ID          int
	Titulo      string
	Descripcion string
	Estado      Estado
	Fecha       time.Time
}

func (t *Tarea) CambiarEstado(nuevo Estado) error {
	if nuevo < Pendiente || nuevo > Cancelada {
		return errors.New("estado inválido")
	}
	t.Estado = nuevo
	return nil
}

func (t Tarea) Mostrar() {
	fmt.Printf("[%d] %s (%s)\n", t.ID, t.Titulo, EstadoToString(t.Estado))
	fmt.Printf("Fecha de creación: %s\n", t.Fecha.Format("2006-01-02 15:04:05"))
	fmt.Println("Descripción:", t.Descripcion)
	fmt.Println("------------------------------------------------------")
}

func EstadoToString(e Estado) string {
	switch e {
	case Pendiente:
		return "Pendiente"
	case EnProgreso:
		return "En progreso"
	case Completada:
		return "Completada"
	case Cancelada:
		return "Cancelada"
	default:
		return "Desconocido"
	}
}
