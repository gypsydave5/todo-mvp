const url = require('url')
const http = require('http')
const qs = require('querystring')
const template = require('./template.html.js')

let todos = []

function add (item) {
  todos.push({ name: item, done: false })
}

function toggle (item) {
  todos = todos.map(todo => todo.name === item
    ? { name: item, done: !todo.done }
    : todo)
}

function remove (item) {
  todos = todos.filter(todo => todo.name !== item)
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

        switch (requestUrl.pathname) {
          case '/done':
            toggle(item)
            break
          case '/not-done':
            toggle(item)
            break
          case '/delete':
            remove(item)
            break
          default:
            add(item)
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
