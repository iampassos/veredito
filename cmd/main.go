package main

import (
	"log"

	"github.com/iampassos/veredito/internal/config"
	"github.com/iampassos/veredito/internal/database"
)

func main() {
	cfg, err := config.Load()
	if err != nil {
		log.Fatalf("Failed to load config: %v", err)
	}
	log.Println("Loaded configuration")

	DB, err := database.Connect(cfg.DBPath)
	if err != nil {
		log.Fatalf("Failed to connect to database: %v", err)
	}
	defer DB.Close()
	log.Println("Connected to database")

	err = initServer()
	if err != nil {
		log.Fatalf("Failed to start server: %v", err)
	}
}
