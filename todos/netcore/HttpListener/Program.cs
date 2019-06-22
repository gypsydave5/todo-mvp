using System;

namespace Todos.HttpListener
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine(System.IO.Directory.GetCurrentDirectory());
            using (var server = new HttpServer("http://127.0.0.1:3000/"))
            {
                server.Start();
                Console.WriteLine("Press any key to stop listening");
                Console.ReadKey();
            }
        }
    }
}
