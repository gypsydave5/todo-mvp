module.exports = todos => `<html>
  <head>
  </head>
  <body>
    <h1>Todo MVP</h1>

    <form method="POST">
      <label for="new-item">New item name</label>
      <input type="text" name="item" required="required" pattern=".{3,}" required title="3 characters minimum"/>
      <input type="submit" value="Add new item"/>
    </form>

    <h2>Todo list</h2>
    <ul>
      ${todos.map(renderTodo).join('\n')}
    </ul>
  </body>
</html>`

function renderTodo (todo) {
  if (todo.done) {
      return `<li class="done"><s>${todo.name}</s> - Done
        <form method="post" action="/done">
          <input type="hidden" name="item" value="${todo.name}"/>
          <input type="submit" formaction="/not-done" value="Mark not done '${todo.name}'" />
          <input type="submit" formaction="/delete" value="Delete '${todo.name}'" />
        </form>
      </li>`
  } else {
      return `<li>${todo.name}
        <form method="post" action="/done">
          <input type="hidden" name="item" value="${todo.name}"/>
          <input type="submit" formaction="/done" value="Mark done '${todo.name}'" />
          <input type="submit" formaction="/delete" value="Delete '${todo.name}'" />
        </form>
      </li>`
  }
}
