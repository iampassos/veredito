package database

import (
	"database/sql"
	_ "github.com/mattn/go-sqlite3"
)

var DB *sql.DB

func Connect(path string) (*sql.DB, error) {
	DB, err := sql.Open("sqlite3", path)
	if err != nil {
		return nil, err
	}

	return DB, nil
}
