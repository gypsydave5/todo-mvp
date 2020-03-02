using System.Collections.Generic;
using System.Threading;

namespace Todos.Common
{
    public class TodoList
    {
        private readonly List<Todo> _todos;
        private int _nextId;

        public TodoList()
        {
            _todos = new List<Todo>();
        }

        public IEnumerable<Todo> Get()
        {
            return _todos;
        }

        public void Add(string name)
        {
            _todos.Add(new Todo() { Id = Interlocked.Increment(ref _nextId), Name = name });
        }

        public void Toggle(int id)
        {
            for (var i = 0; i < _todos.Count; i++)
            {
                var todo = _todos[i];
                if (todo.Id == id)
                {
                    todo.Done = !todo.Done;
                    _todos[i] = todo;
                    break;
                }
            }
        }

        public void Remove(int id)
        {
            var index = -1;

            for (var i = 0; i < _todos.Count; i++)
            {
                var todo = _todos[i];
                if (todo.Id == id)
                {
                    index = i;
                    break;
                }
            }

            if (index >= 0)
            {
                _todos.RemoveAt(index);
            }
        }
    }
}
