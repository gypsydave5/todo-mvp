const url = require('url')
const http = require('http')
const qs = require('querystring')
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
  switch (request.method) {
    case 'GET':
      return response.end(template(todos))
    case 'POST':
      parseForm(request).then(formdata => {
        const requestUrl = url.parse(request.url, true)
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
          default:
            add(name)
            break
        }
        redirectToHome(response)
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

const port = 3000
server.on('error', e => {
  console.error(`Server error: ${e}\nShutting down`)
  process.exit(1)
})

server.on('listening', e => {
  console.log(`Listening on port ${port}`)
})

server.listen(port)
