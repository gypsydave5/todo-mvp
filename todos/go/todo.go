package main

import (
	"net/http"
	"html/template"
)

type Todo struct {
	Name string
	Done bool
}

type TodoListServer struct {
	items []Todo
	tmpl  *template.Template
}

func (t *TodoListServer) CheckItem(name string) {
	for i, item := range t.items {
		if item.Name == name {
			t.items[i].Done = true
		}
	}
}

func (t *TodoListServer) AddItem(name string) {
	t.items = append(t.items, Todo{name, false})
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
		if r.URL.Path == "/done" {
			t.CheckItem(r.FormValue("item"))
			t.RedirectToHome(w)
		} else {
			t.AddItem(r.FormValue("new-item"))
			t.RedirectToHome(w)
		}
	}
}

func main() {
	server := &TodoListServer{
		tmpl: template.Must(template.ParseFiles("template.html")),
	}

	http.ListenAndServe(":4000", server)
}
