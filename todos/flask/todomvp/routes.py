from flask import render_template, redirect, url_for
from flask.globals import request
from todomvp import app

todos = [
    {
        "id": 1,
        "name": "Make TODOs Great Again",
        "done": False
    }
]

def get_next_id() -> int:
    """WARNING: Not thread-safe, and shouldn't be used in production. I'm
    basically mimicking basic DB utility functions since I don't have any.

    Use a DB, and use their utility functions!"""
    if len(todos) == 0:
        return 0
    else:
        return max([t["id"] for t in todos]) + 1


@app.route("/", methods=["GET", "POST"])
def index():
    if request.method == "POST":
        todo = {
            "id": get_next_id(),
            "name": request.form.get("item"),
            "done": False
        }
        todos.append(todo)

    return render_template("home.html", todos=todos)



def get_todo_index() -> int:
    return [t["id"] for t in todos].index(int(request.form.get("item")))


@app.route("/done", methods=["POST"])
def done():
    idx = get_todo_index()
    todos[idx]["done"] = True
    return redirect(url_for("index"))


@app.route("/not-done", methods=["POST"])
def not_done():
    idx = get_todo_index()
    todos[idx]["done"] = False
    return redirect(url_for("index"))


@app.route("/delete", methods=["POST"])
def delete():
    idx = get_todo_index()
    del todos[idx]
    return redirect(url_for("index"))
