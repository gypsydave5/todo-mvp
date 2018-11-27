from django.views.generic import TemplateView
from django.http import HttpResponse, HttpRequest
from django.shortcuts import redirect, reverse

todos = [
  {
    "id": 1,
    "name": "Make TODOs Great Again!",
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


class IndexView(TemplateView):
  # If GET, will render template. If POST, will use the post function below.
  # All other methods aren't allowed.
  template_name = "home.html"
  
  def get_context_data(self, **kwargs):
    context = super().get_context_data(**kwargs)
    context["todos"] = todos
    return context

  def post(self, request: HttpRequest, *args, **kwargs):
    todo = {
      "id": get_next_id(),
      "name": request.POST.get("item"),
      "done": False
    }
    todos.append(todo)

    return redirect(reverse("index"))

