package main

import "fmt"

func main() {
	// Declarando una variable
	var cat, dog string

	// Asignado el valores
	dog = "🐕‍🦺"
	cat = "🐈‍⬛"

	// Imprimir en consola los valores
	fmt.Println("Gato:", cat, "\nPerro:", dog)

	// Declaración de variables enteras
	// 3 Formas de declarar variables en Go
	var area int
	var altura int = 14
	base := 12

	fmt.Println(base, altura, area)
}
