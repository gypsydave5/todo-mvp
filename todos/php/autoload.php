<?php
/**
 * Project-specific auto-loading implementation.
 *
 * After registering this autoload function with SPL, the namespaces on the left of the loaders array will load classes found in the paths on the right.
 *
 * @param string $class The fully-qualified class name.
 * @return void
 */

 spl_autoload_register(function ($class) {
    $loaders = [
        'Todo\\PHP\\' => '/classes/'
    ];

    foreach($loaders as $prefix => $base_dir){
        $len = strlen($prefix);
        if(strncmp($prefix, $class,$len) !== 0){
            continue;
        }
        $relative_class = substr($class, $len);

        $file = __DIR__ . $base_dir .
        str_replace('\\', '/',
            $relative_class) .
        '.php';

        if (file_exists($file)) {
            require $file;
            return;
        }
    } //end foreach
});
