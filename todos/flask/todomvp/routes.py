from flask import render_template
from todomvp import app

todos = [
    {
        "id": 1,
        "name": "Make TODOs Great Again",
        "done": False
    }
]


@app.route("/")
def index():
    return render_template("home.html", todos=todos)
