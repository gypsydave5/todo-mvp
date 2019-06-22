using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Net;
using System.Threading.Tasks;
using System.Web;
using Todos.Common;

namespace Todos.HttpListener
{
    public class HttpServer : IDisposable
    {
        private readonly System.Net.HttpListener _listener;
        private readonly TodoList _todos;
        private readonly string _prefix;

        public HttpServer(string prefix)
        {
            try
            {
                _listener = new System.Net.HttpListener();
                _prefix = prefix;
                _todos = new TodoList();
            }
            catch (PlatformNotSupportedException)
            {
                throw;
            }
        }

        public async void Start()
        {
            _listener.Prefixes.Add(_prefix);
            _listener.Start();

            Console.WriteLine($"Started listening on {_prefix}");

            while (_listener.IsListening)
            {
                try
                {
                    var context = await _listener.GetContextAsync();
                    var stopwatch = new Stopwatch();
                    Console.WriteLine($"Request starting HTTP/{context.Request.ProtocolVersion} {context.Request.HttpMethod} {context.Request.Url}");
                    stopwatch.Start();
                    await ProcessRequest(context);
                    stopwatch.Stop();
                    Console.WriteLine($"Request finished in {stopwatch.Elapsed.TotalMilliseconds:0.####}ms {context.Response.StatusCode} {context.Response.ContentType}");
                }
                catch(Exception ex)
                {
                    Console.WriteLine(ex.Message);
                }
            }
        }

        public void Dispose()
        {
            if (_listener != null && _listener.IsListening)
            {
                _listener.Stop();
            }
        }

        private async Task ProcessRequest(HttpListenerContext context)
        {
            switch (context.Request.HttpMethod)
            {
                case "GET":
                    if (context.Request.RawUrl.Equals("/"))
                    {
                        await HandleIndex(context);
                        break;
                    }

                    if (context.Request.RawUrl.StartsWith("/static"))
                    {
                        await HandleStatic(context);
                        break;
                    }

                    Handle404(context);
                    break;

                case "POST":
                    await HandlePost(context);
                    RedirectToHome(context);
                    break;

                default:
                    RedirectToHome(context);
                    break;
            }

            context.Response.Close();
        }

        private async Task HandlePost(HttpListenerContext context)
        {
            using (var reader = new StreamReader(context.Request.InputStream))
            {
                var formData = HttpUtility.ParseQueryString(await reader.ReadToEndAsync());
                var item = formData["item"];
                int.TryParse(item, out var id);

                switch (context.Request.RawUrl)
                {
                    case "/done":
                    case "/not-done":
                        _todos.Toggle(id);
                        break;

                    case "/delete":
                        _todos.Remove(id);
                        break;

                    case "/":
                        _todos.Add(item);
                        break;
                }
            }
        }

        private async Task HandleStatic(HttpListenerContext context)
        {
            var mimeTypes = new Dictionary<string, string>() { { "css", "text/css" }, { "svg", "image/svg+xml" } };
            // When running in Visual studio current directory is set to bin of Debug or Release
            // When running from dotnet command current directory is set tu folder you run command
            //var filePath = $"../../../../{context.Request.RawUrl}";
            var filePath = $".{context.Request.RawUrl}";
            var extension = filePath.Split('.').Last();
            try
            {
                using (var file = File.OpenRead(filePath))
                {
                    context.Response.StatusCode = 200;
                    context.Response.ContentType = mimeTypes[extension];
                    await file.CopyToAsync(context.Response.OutputStream);
                }
            }
            catch
            {
                Handle404(context);
            }
        }

        private async Task HandleIndex(HttpListenerContext context)
        {
            var index = IndexTemplate.Render(_todos.Get());
            context.Response.ContentType = "text/html";
            context.Response.StatusCode = 200;
            await context.Response.OutputStream.WriteAsync(index, 0, index.Length);
        }

        private void Handle404(HttpListenerContext context)
        {
            context.Response.StatusCode = 404;
        }

        private void RedirectToHome(HttpListenerContext context)
        {
            context.Response.StatusCode = 303;
            context.Response.RedirectLocation = "/";
        }
    }
}
