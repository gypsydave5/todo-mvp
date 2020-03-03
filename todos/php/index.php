<?php
/* Web application entry point and bootstrapping.
 */

use Todo\PHP\todo;
use Todo\PHP\controller;

require_once('autoload.php');

$controller = new controller($_POST, $_GET);
$template = '';

// Grab our passed URL parameters (pretty URLs).
$url = $_GET['url'] ?? $_SERVER['PATH_INFO'] ?? "";

// Parse the controller action out of the url (first part before first slash). The rest is the 'parameters'.
$parameters = explode("/", ltrim($url, '/'));
$action = $parameters[0];
$parameters = array_shift($parameters);
if(empty($action)) $action = 'index';


// Initialize todo list object.
if (empty($_POST) || !array_key_exists('current-list', $_POST)) {
    $todoList = new todo(); // No persistence, so we start a new instance.
} else {
    $todoList = new todo(json_decode($_POST['current-list'], true));
}

// Very basic routing.
if ((int)method_exists($controller, $action)) {
    // Call controller method and inject $todoList object.
    // In a full application we'd also pass the $parameters values to the controller actions.
    $template = call_user_func_array(array($controller, $action), array(&$todoList));
} else {
    http_response_code(404);
    echo("404 Page not found. <a href='/'>Return home.</a>");
    exit();
}

// Set up values for our 'template':
$todos = $todoList->getItems();
$currentList = htmlspecialchars($todoList->getItemsJSON());

// Include our html page template, which uses the above values.
require_once($template.'.tpl.php');
