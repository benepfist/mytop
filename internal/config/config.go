package config

import (
	"os"
	"strconv"

	"mytop/internal/app"
)

// Loader lädt Konfiguration aus Umgebungsvariablen und stellt Defaults bereit.
type Loader struct{}

func NewLoader() *Loader {
	return &Loader{}
}

func (l *Loader) Load() (app.Config, error) {
	port := 3306
	if rawPort := os.Getenv("MYTOP_PORT"); rawPort != "" {
		parsed, err := strconv.Atoi(rawPort)
		if err != nil {
			return app.Config{}, err
		}
		port = parsed
	}

	cfg := app.Config{
		Host: envOrDefault("MYTOP_HOST", "127.0.0.1"),
		Port: port,
		User: envOrDefault("MYTOP_USER", "root"),
	}

	return cfg, nil
}

func envOrDefault(key, fallback string) string {
	if value := os.Getenv(key); value != "" {
		return value
	}
	return fallback
}
