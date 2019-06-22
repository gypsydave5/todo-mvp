using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Builder;
using Microsoft.AspNetCore.Hosting;
using Microsoft.AspNetCore.Http;
using Microsoft.Extensions.DependencyInjection;
using Todos.Common;

namespace AspNet
{
    public class Startup
    {
        // This method gets called by the runtime. Use this method to add services to the container.
        // For more information on how to configure your application, visit https://go.microsoft.com/fwlink/?LinkID=398940
        public void ConfigureServices(IServiceCollection services)
        {
            services.AddSingleton(new TodoList());
        }

        // This method gets called by the runtime. Use this method to configure the HTTP request pipeline.
        public void Configure(IApplicationBuilder app, IHostingEnvironment env)
        {
            if (env.IsDevelopment())
            {
                app.UseDeveloperExceptionPage();
            }

            app.Run(async (context) =>
            {
                switch(context.Request.Method)
                {
                    case "GET":
                        if(context.Request.Path.Value.Equals("/"))
                        {
                            await HandleIndex(context);
                            break;
                        }

                        if (context.Request.Path.Value.StartsWith("/static"))
                        {
                            await HandleStatic(context);
                            break;
                        }

                        Handle404(context);
                        break;

                    case "POST":
                        HandlePost(context);
                        RedirectToHome(context);
                        break;

                    default:
                        RedirectToHome(context);
                        break;
                }
            });
        }

        private void HandlePost(HttpContext context)
        {
            var todos = context.RequestServices.GetService<TodoList>();
            var formData = context.Request.Form;
            var item = formData["item"];
            int.TryParse(item, out var id);

            switch (context.Request.Path)
            {
                case "/done":
                case "/not-done":
                    todos.Toggle(id);
                    break;

                case "/delete":
                    todos.Remove(id);
                    break;

                case "/":
                    todos.Add(item);
                    break;
            }
        }

        private async Task HandleIndex(HttpContext context)
        {
            var todos = context.RequestServices.GetService<TodoList>();
            var index = IndexTemplate.Render(todos.Get());
            context.Response.ContentType = "text/html";
            context.Response.StatusCode = 200;
            await context.Response.Body.WriteAsync(index, 0, index.Length);
        }

        private async Task HandleStatic(HttpContext context)
        {
            var mimeTypes = new Dictionary<string, string>() { { "css", "text/css" }, { "svg", "image/svg+xml" } };
            var filePath = $"../{context.Request.Path}";
            var extension = filePath.Split('.').Last();
            try
            {
                using (var file = File.OpenRead(filePath))
                {
                    context.Response.StatusCode = 200;
                    context.Response.ContentType = mimeTypes[extension];
                    await file.CopyToAsync(context.Response.Body);
                }
            }
            catch
            {
                Handle404(context);
            }
        }

        private void Handle404(HttpContext context)
        {
            context.Response.StatusCode = 404;
        }

        private void RedirectToHome(HttpContext context)
        {
            context.Response.StatusCode = 303;
            context.Response.Headers.Add("Location", "/");
        }
    }
}
