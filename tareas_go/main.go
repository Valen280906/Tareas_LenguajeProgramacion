package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
	"tareas_go/commands"
	"tareas_go/estructuras"
)

func main() {
	gestor := commands.NuevoGestor()
	reader := bufio.NewReader(os.Stdin)

	fmt.Println("=== SISTEMA DE GESTIÓN DE TAREAS (CLI) ===")
	fmt.Println("Comandos disponibles:")
	fmt.Println("- agregar <título> | <descripción>")
	fmt.Println("- listar")
	fmt.Println("- enprogreso <id>")
	fmt.Println("- completar <id>")
	fmt.Println("- cancelar <id>")
	fmt.Println("- eliminar <id> o eliminar todo")
	fmt.Println("- ayuda")
	fmt.Println("- salir")

	for {
		fmt.Print("\n> ")
		input, err := reader.ReadString('\n')
		if err != nil {
			fmt.Println("Error al leer la entrada:", err)
			continue
		}

		input = strings.TrimSpace(input)
		if input == "" {
			continue
		}

		args := strings.SplitN(input, " ", 2)
		comando := strings.ToLower(args[0])

		switch comando {

		case "agregar":
			if len(args) < 2 {
				fmt.Println("Uso: agregar <título> | <descripción>")
				continue
			}
			partes := strings.SplitN(args[1], "|", 2)
			if len(partes) < 2 {
				fmt.Println("Usa el separador '|' entre título y descripción.")
				continue
			}
			titulo := strings.TrimSpace(partes[0])
			desc := strings.TrimSpace(partes[1])
			t := gestor.AgregarTarea(titulo, desc)
			fmt.Printf("Tarea creada correctamente: [%d] %s\n", t.ID, t.Titulo)

		case "listar":
			gestor.ListarTareas()

		case "enprogreso":
			if len(args) < 2 {
				fmt.Println("Uso: enprogreso <id>")
				continue
			}
			id, _ := strconv.Atoi(args[1])
			err := gestor.CambiarEstado(id, estructuras.EnProgreso)
			if err != nil {
				fmt.Println("Error:", err)
			}

		case "completar":
			if len(args) < 2 {
				fmt.Println("Uso: completar <id>")
				continue
			}
			id, _ := strconv.Atoi(args[1])
			err := gestor.CambiarEstado(id, estructuras.Completada)
			if err != nil {
				fmt.Println("Error:", err)
			}

		case "cancelar":
			if len(args) < 2 {
				fmt.Println("Uso: cancelar <id>")
				continue
			}
			id, _ := strconv.Atoi(args[1])
			err := gestor.CambiarEstado(id, estructuras.Cancelada)
			if err != nil {
				fmt.Println("Error:", err)
			}

		case "eliminar":
			if len(args) < 2 {
				fmt.Println("Uso: eliminar <id>  o  eliminar todo")
				continue
			}

			arg := strings.ToLower(strings.TrimSpace(args[1]))

			if arg == "todo" {
				fmt.Print("¿Seguro que deseas eliminar todas las tareas? (s/n): ")
				confirm, _ := reader.ReadString('\n')
				confirm = strings.TrimSpace(strings.ToLower(confirm))

				if confirm == "s" {
					err := gestor.EliminarTodasLasTareas()
					if err != nil {
						fmt.Println("Error:", err)
					}
				} else {
					fmt.Println("Operación cancelada.")
				}

			} else {
				id, err := strconv.Atoi(arg)
				if err != nil {
					fmt.Println("El ID debe ser un número entero o 'todo'.")
					continue
				}
				err = gestor.EliminarTarea(id)
				if err != nil {
					fmt.Println(err)
				} else {
					fmt.Println("Tarea eliminada correctamente.")
				}
			}

		case "ayuda":
			fmt.Println("Comandos disponibles:")
			fmt.Println("- agregar <título> | <descripción>")
			fmt.Println("- listar")
			fmt.Println("- enprogreso <id>")
			fmt.Println("- completar <id>")
			fmt.Println("- cancelar <id>")
			fmt.Println("- eliminar <id>")
			fmt.Println("- salir")

		case "salir":
			fmt.Println("Guardando cambios y saliendo del gestor de tareas...")
			gestor.GuardarTareasJSON()
			return

		default:
			fmt.Println("Comando no reconocido. Escribe 'ayuda' para ver las opciones.")
		}
	}
}
