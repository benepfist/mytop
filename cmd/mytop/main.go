package main

import (
	"log"
	"os"

	"mytop/internal/app"
	"mytop/internal/config"
	"mytop/internal/tui"
)

func main() {
	cfgLoader := config.NewLoader()
	ui := tui.New()
	db := app.NewMemoryDB()

	application := app.New(cfgLoader, ui, db)
	if err := application.Run(os.Args[1:]); err != nil {
		log.Fatal(err)
	}
}
