package main

import (
	"log"
	"net/http"
)

func initServer() error {
	router := http.NewServeMux()

	router.HandleFunc("GET /", homeHandler)

	log.Println("Starting server on port 3000")

	err := http.ListenAndServe(":3000", loggingMiddleware(router))

	if err != nil {
		return err
	}

	return nil
}

func loggingMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		log.Printf("%v Request at %v", r.Method, r.URL.Path)
		next.ServeHTTP(w, r)
	})
}

func homeHandler(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusOK)
}
