const url = require('url')
const http = require('http')
const qs = require('querystring')
const fs = require('fs')
const template = require('./template.html.js')

let todos = []

const nextId = (() => {
  let next = 0
  return () => next++
})()

function add (name) {
  todos.push({ name: name, done: false, id: nextId() })
}

function toggle (id) {
  todos = todos.map(todo => todo.id === id
    ? { name: todo.name, done: !todo.done, id: id }
    : todo)
}

function remove (id) {
  todos = todos.filter(todo => todo.id !== id)
}

const server = http.createServer()

server.on('request', (request, response) => {
  const requestUrl = url.parse(request.url, true)
  if (onPath('/static', requestUrl)) {
    return handleStatic(request, response)
  }

  switch (request.method) {
    case 'GET':
      if (onPath('/', requestUrl)) {
        return response.end(template(todos))
      }
      return fourOhFour(response)

    case 'POST':
      parseForm(request).then(formdata => {
        const item = formdata.item
        const name = item
        const id = parseInt(item)

        switch (requestUrl.pathname) {
          case '/done':
            toggle(id)
            break
          case '/not-done':
            toggle(id)
            break
          case '/delete':
            remove(id)
            break
          case '/':
            add(name)
            break
          default:
            return fourOhFour(response)
        }
        return redirectToHome(response)
      })
  }
})

function parseForm (request) {
  return new Promise((resolve, reject) => {
    const chunks = []

    request.on('data', chunk => chunks.push(chunk))

    request.on('end', () => {
      const body = Buffer.concat(chunks).toString()
      resolve(qs.parse(body))
    })
  })
}

function redirectToHome (response) {
  response.statusCode = 303
  response.setHeader('Location', '/')
  response.end()
}

function onPath (path, requestUrl) {
  return requestUrl.pathname.startsWith(path)
}

function handleStatic (request, response) {
  const mimetypes = { 'css': 'text/css', 'svg': 'application/image/svg+xml' }
  const filePath = '.' + request.url
  const extension = filePath.split('.').pop()
  fs.readFile(filePath, (error, content) => {
    if (error) return fourOhFour(response)
    response.writeHead(200, { 'Content-Type': mimetypes[extension] })
    response.end(content)
  })
}

function fourOhFour (response) {
  response.writeHead(404)
  response.end('Page not found')
}

const port = 3000
server.on('error', e => {
  console.error(`Server error: ${e}\nShutting down`)
  process.exit(1)
})

server.on('listening', e => {
  console.log(`Listening on port ${port}`)
})

server.listen(port)
