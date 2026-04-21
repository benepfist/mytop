package main

import (
	"log"

	"mytop/internal/app"
	"mytop/internal/config"
	"mytop/internal/tui"
)

func main() {
	cfgLoader := config.NewLoader()
	ui := tui.New()

	application := app.New(cfgLoader, ui)
	if err := application.Run(); err != nil {
		log.Fatal(err)
	}
}
