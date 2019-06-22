# TODO-MVP in .NET CORE (C#)

This implementation is written in [.NET Core] using two different HTTP servers that are available in the framework

  - [ASP.NET Core] cross platform HTTP server ([Kestrel])
  - [HTTP.sys] windows only HTTP server ([HttpListener])

## Prerequisites
   Install runtime or sdk from [.NET Core download] website

## Run host

From todo-mvp/todos/netcore folder run desired host

#### HttpListener

This will work on Windows

    dotnet run --project HttpListener\HttpListener.csproj

#### Kestrel

This will work on Windows, Linux, macOS

    dotnet run --project AspNet\AspNet.csproj

## Go to website
Access url [http://localhost:3000/](http://localhost:3000/)

Enjoy

[.NET Core]: https://docs.microsoft.com/en-us/dotnet/core/
[.NET Core download]: https://dotnet.microsoft.com/download
[ASP.NET Core]: https://docs.microsoft.com/en-us/aspnet/core/fundamentals/servers/index?view=aspnetcore-2.2
[HttpListener]: https://docs.microsoft.com/en-us/dotnet/framework/network-programming/httplistener
[Kestrel]: https://docs.microsoft.com/en-us/aspnet/core/fundamentals/servers/kestrel?view=aspnetcore-2.2
[HTTP.sys]: https://docs.microsoft.com/en-us/aspnet/core/fundamentals/servers/httpsys?view=aspnetcore-2.2
