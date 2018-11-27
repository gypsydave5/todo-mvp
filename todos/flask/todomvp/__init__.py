import os

from flask import Flask

app = Flask(__name__)
app.config["SECRET_KEY"] = os.environ.get("FLASK_SECRET")

from todomvp import routes

if __name__ == "__main__":
    app.run(debug=bool(os.environ.get("FLASK_DEBUG")))
