package main

import (
	"log"
	"net/http"

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

	router := http.NewServeMux()

	router.HandleFunc("GET /", HomeHandler)

	log.Println("Starting server on port 8080")
	http.ListenAndServe(":8080", router)
}

func HomeHandler(rw http.ResponseWriter, r *http.Request) {
	log.Println("Received request at /")

	rw.WriteHeader(http.StatusOK)
}
