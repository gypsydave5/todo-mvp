package main

import (
	"fmt"
	"html/template"
	"log"
	"net/http"
	"strconv"
)

type Todo struct {
	Id   int
	Name string
	Done bool
}

type TodoListServer struct {
	items  []Todo
	nextId int
	tmpl   *template.Template
}

func (t *TodoListServer) Check(id int) {
	for i, item := range t.items {
		if item.Id == id {
			t.items[i].Done = true
		}
	}
}

func (t *TodoListServer) UnCheck(id int) {
	for i, item := range t.items {
		if item.Id == id {
			t.items[i].Done = false
		}
	}
}

func (t *TodoListServer) Delete(id int) {
	var newList []Todo
	for _, item := range t.items {
		if item.Id != id {
			newList = append(newList, item)
		}
	}
	t.items = newList
}

func (t *TodoListServer) Add(name string) {
	t.items = append(t.items, Todo{t.nextId, name, false})
	t.nextId++
}

func (t *TodoListServer) RedirectToHome(w http.ResponseWriter) {
	w.Header().Add("location", "/")
	w.WriteHeader(http.StatusSeeOther)
}

func (t *TodoListServer) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	switch r.Method {
	case http.MethodGet:
		t.tmpl.Execute(w, t.items)
	case http.MethodPost:
		item := r.FormValue("item")
		id, _ := strconv.Atoi(item)
		switch r.URL.Path {
		case "/done":
			t.Check(id)
		case "/not-done":
			t.UnCheck(id)
		case "/delete":
			t.Delete(id)
		default:
			t.Add(item)
		}
		t.RedirectToHome(w)
	}
}

func main() {
	mux := http.NewServeMux()

	mux.Handle("/static/", http.StripPrefix("/static/", http.FileServer(http.Dir("static"))))
	mux.Handle("/", &TodoListServer{
		tmpl: template.Must(template.ParseFiles("template.html")),
	})

	fmt.Println("Listening on port 3000")
	log.Fatal(http.ListenAndServe(":3000", mux))
}
