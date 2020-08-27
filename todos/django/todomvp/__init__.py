# Note: I am manually creating a tiny project here since Django is much more
# powerful and a bit too much of a powerhouse for this tiny project ;)
# Django's new project command adds a whole lot more stuff that make sense for a
# typical server, but not here.
import os

from todomvp.views import IndexView, todo_done, todo_undone, todo_delete
from django.urls import path

DEBUG = True
SECRET_KEY = '4l0ngs3cr3tstr1ngw3lln0ts0l0ngw41tn0w1tsl0ng3n0ugh'
ROOT_URLCONF = __name__
BASE_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
TEMPLATES = [
    {
        'BACKEND': 'django.template.backends.django.DjangoTemplates',
        'DIRS': [os.path.join(BASE_DIR, "templates")]
    },
]

INSTALLED_APPS = [
    "django.contrib.staticfiles"
]

STATIC_URL = "/static/"
STATICFILES_FINDERS = [
    'django.contrib.staticfiles.finders.FileSystemFinder'
]
STATICFILES_DIRS = [
    os.path.join(BASE_DIR, "static")
]

urlpatterns = [
    path("", IndexView.as_view(), name="index"),
    path("done", todo_done, name="done"),
    path("not-done", todo_undone, name="not-done"),
    path("delete", todo_delete, name="delete")
]
