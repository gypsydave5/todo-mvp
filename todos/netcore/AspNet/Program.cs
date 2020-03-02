using System;
using Microsoft.AspNetCore;
using Microsoft.AspNetCore.Hosting;

namespace AspNet
{
    public class Program
    {
        public static void Main(string[] args)
        {
            CreateWebHostBuilder(args).Build().Run();
            Console.WriteLine(System.IO.Directory.GetCurrentDirectory());
        }

        public static IWebHostBuilder CreateWebHostBuilder(string[] args) =>
            WebHost.CreateDefaultBuilder(args)
                .UseStartup<Startup>()
                .UseUrls("http://127.0.0.1:3000");
    }
}
