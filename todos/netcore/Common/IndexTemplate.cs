using System.Collections.Generic;
using System.Text;

namespace Todos.Common
{
    public static class IndexTemplate
    {
        public static byte[] Render(IEnumerable<Todo> todos)
        {
            var builder = new StringBuilder();

            builder.Append(MainTemplateStart);
            foreach(var todo in todos)
            {
                builder.Append(ToDoTemplate(todo));
            }
            builder.Append(MainTemplateEnd);
            return Encoding.UTF8.GetBytes(builder.ToString());
        }

        private static string MainTemplateStart =>
            @"<!DOCTYPE html>
            <html>
                <head>
                    <title>Todo MVP</title>
                    <meta charset='utf-8'>
                    <link rel='stylesheet' href='/static/todo.css'>
                </head>
                <body>
                    <h1>Todo MVP</h1>
                    <section class='new-todo'>
                        <form method='POST'>
                        <input type='text'
                                id='new-item'
                                name='item'
                                pattern='.{{3,}}'
                                required
                                aria-label='Write a new todo item'
                                title='3 characters minimum'/>
                        <input type='submit'
                                value='Add new item'
                                id='add-new-item'/>
                        </form>
                    </section>
                    <section class='items'>
                        <h2>Todo list</h2>
                        <ul>";

        private static string MainTemplateEnd =>
            @"</ul>
                </section>
                </body>
            </html>";

        private static string ToDoTemplate(Todo todo)
        {
            var toDoClass = todo.Done ? "todo done" : "todo";
            var name = todo.Done ? $"<s>{todo.Name}</s>" : todo.Name;
            var markAction = todo.Done ? "/not-done" : "/done";
            var markClass = todo.Done ? "uncomplete" : "complete";
            var markValue = todo.Done ? $"Mark not done \"{todo.Name}\"" : $"Mark done \"{todo.Name}\"";

            return $@"<li class='{toDoClass}'>
                        <span class='item-name'>
                            {name}
                        </span>
                        <form method='post' action='{markAction}'>
                            <input type='hidden' name='item' value='{todo.Id}'/>
                            <input class='{markClass}'
                                    type='submit'
                                    value='{markValue}' />
                        </form>
                        <form method='post' action='/delete'>
                            <input type='hidden' name='item' value='{todo.Id}'/>
                            <input class='delete'
                                    type='submit'
                                    value='Delete ""{todo.Name}""' />
                        </form>
                    </li>";
        }
    }
}
