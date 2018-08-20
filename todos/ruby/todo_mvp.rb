require 'rack'
require 'erb'

class Todo
  attr_reader :name, :done

  def initialize name
    @name = name
    @done = false
  end

  def toggle
    @done = !@done
    self
  end
end

class TodoList
  @@template = ERB.new File.read("./pages/todo_list.html.erb")

  attr_reader :todos

  def initialize
    @todos = []
  end

  def add name
    @todos.push(Todo.new(name))
  end

  def toggle name
    @todos = @todos.map do |todo|
      if todo.name == name
        todo.toggle
      else
        todo
      end
    end
  end

  def render
    @@template.result(binding)
  end
end

# The todo 'API' - global (for now)
$todos = TodoList.new

# Page - global for now - could be wrapped up
$new_todo_page = ERB.new File.read("./pages/new_todo.html.erb")

# Here's a simple top level router - could be a class if it implements 'call'
def new_router
  app = Proc.new do |env|
    request = Rack::Request.new(env)
    routes(request).finish
  end

  Rack::Static.new(app, :urls => ["/css"], :root => "public")
end

def routes request
  puts "PATH: #{request.path}"
  case request.path
  when "/todo"
    todo_router request
  when "/new-todo"
    new_todo_router request
  when "/toggle-todo"
    toggle_todo_router request
  else
    not_found
  end
end

def toggle_todo_router request
  if request.get?
    return toggle_todo_handler request
  end

  invalid_method
end

def todo_router request
  if request.get?
    return show_todos_handler(request)
  end

  invalid_method
end

def new_todo_router request
  if request.get?
    return show_add_todo_form_handler request
  end

  if request.post?
    return add_todo_handler request
  end

  invalid_method
end

def show_add_todo_form_handler request
  response = Rack::Response.new
  response.status = 200
  response.set_header 'Content-Type', 'text/html'
  response.write $new_todo_page.result
  response
end

def add_todo_handler request
  name = request[:name]
  $todos.add name
  redirect "/todo"
end

def show_todos_handler request
  response = Rack::Response.new
  response.status = 200
  response.set_header 'Content-Type', 'text/html'
  response.write $todos.render
  response
end

def toggle_todo_handler request
  puts request.params
  name = request[:name]
  $todos.toggle name
  redirect "/todo"
end

def not_found
  response = Rack::Response.new
  response.status = 404
  response
end

def invalid_method
  response = Rack::Response.new
  response.status = 405
  response
end

def redirect url
  response = Rack::Response.new
  response.redirect(url)
  response
end

Rack::Handler::WEBrick.run new_router, Port: 4545