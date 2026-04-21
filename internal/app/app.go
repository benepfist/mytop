package app

import "fmt"

// Config beschreibt die minimalen Laufzeitoptionen für den ersten Rewrite-Stand.
type Config struct {
	Host string
	Port int
	User string
}

// ConfigLoader kapselt das Einlesen und spätere Erweitern der Konfiguration
// (Datei, Env, CLI-Flags).
type ConfigLoader interface {
	Load() (Config, error)
}

// UI definiert die Schnittstelle zwischen Applikationslogik und TUI.
type UI interface {
	Init(cfg Config) error
	Run() error
}

// App orchestriert Konfiguration und UI.
type App struct {
	cfgLoader ConfigLoader
	ui        UI
}

func New(cfgLoader ConfigLoader, ui UI) *App {
	return &App{
		cfgLoader: cfgLoader,
		ui:        ui,
	}
}

func (a *App) Run() error {
	cfg, err := a.cfgLoader.Load()
	if err != nil {
		return fmt.Errorf("load config: %w", err)
	}

	if err := a.ui.Init(cfg); err != nil {
		return fmt.Errorf("init ui: %w", err)
	}

	if err := a.ui.Run(); err != nil {
		return fmt.Errorf("run ui: %w", err)
	}

	return nil
}
