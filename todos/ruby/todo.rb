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
  @@template = ERB.new File.read("./template.html.erb")

  attr_reader :todos

  def initialize
    @todos = []
  end

  def add name
    @todos.push(Todo.new(name))
  end

  def delete name
    @todos.delete_if {|todo| todo.name == name}
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

# Here's a simple top level router - could be a class if it implements 'call'

def routes request
  case request.request_method
  when 'GET'
    show_todos_handler
  when 'POST'
    item = request[:item]
    case request.path
    when '/done'
      $todos.toggle item
    when '/not-done'
      $todos.toggle item
    when '/delete'
      $todos.delete item
    else
      $todos.add item
    end
    redirect "/"
  else
    redirect "/"
  end
end

def show_todos_handler
  response = Rack::Response.new
  response.status = 200
  response.set_header 'Content-Type', 'text/html'
  response.write $todos.render
  response
end

def redirect url
  response = Rack::Response.new
  response.redirect(url)
  response
end

def new_server
  app = Proc.new do |env|
    request = Rack::Request.new(env)
    routes(request).finish
  end

  # to statically serve CSS
  Rack::Static.new(app, :urls => ["/css"], :root => "public")
end

Rack::Handler::WEBrick.run new_server, Port: 3000