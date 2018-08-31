package main

import (
	"html/template"
	"net/http"
)

type Todo struct {
	Name string
	Done bool
}

type TodoListServer struct {
	items []Todo
	tmpl  *template.Template
}

func (t *TodoListServer) Check(name string) {
	for i, item := range t.items {
		if item.Name == name {
			t.items[i].Done = true
		}
	}
}

func (t *TodoListServer) UnCheck(name string) {
	for i, item := range t.items {
		if item.Name == name {
			t.items[i].Done = false
		}
	}
}

func (t *TodoListServer) Delete(name string) {
	var newList []Todo
	for _, item := range t.items {
		if item.Name != name {
			newList = append(newList, item)
		}
	}
	t.items = newList
}

func (t *TodoListServer) Add(name string) {
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
		item := r.FormValue("item")
		switch r.URL.Path {
		case "/done":
			t.Check(item)
		case "/not-done":
			t.UnCheck(item)
		case "/delete":
			t.Delete(item)
		default:
			t.Add(item)
		}
		t.RedirectToHome(w)
	}
}

func main() {
	server := &TodoListServer{
		tmpl: template.Must(template.ParseFiles("template.html")),
	}

	http.ListenAndServe(":3000", server)
}
