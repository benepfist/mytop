package tui

import (
	"fmt"

	"mytop/internal/app"
)

// TUI ist ein minimaler Platzhalter für den schrittweisen Rewrite.
type TUI struct {
	cfg app.Config
}

func New() *TUI {
	return &TUI{}
}

func (t *TUI) Init(cfg app.Config) error {
	t.cfg = cfg
	return nil
}

func (t *TUI) Run() error {
	fmt.Printf("mytop-go bootstrap: connecting to %s:%d as %s\n", t.cfg.Host, t.cfg.Port, t.cfg.User)
	return nil
}
