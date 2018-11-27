# Note: I am manually creating a tiny project here since Django is much more
# powerful and a bit too much of a powerhouse for this tiny project ;)
# Django's new project command adds a whole lot more stuff that make sense for a
# typical server, but not here.
import os

from todomvp.views import IndexView
from django.urls import path

DEBUG = True
SECRET_KEY = '4l0ngs3cr3tstr1ngw3lln0ts0l0ngw41tn0w1tsl0ng3n0ugh'
ROOT_URLCONF = __name__
BASE_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
TEMPLATES = [
    {
        'BACKEND': 'django.template.backends.django.DjangoTemplates',
        'DIRS': [os.path.join(BASE_DIR, "todomvp/templates")]
    },
]


urlpatterns = [
    path("", IndexView.as_view(), name="index")
]
