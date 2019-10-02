<?php
namespace Todo\PHP;

/* A very basic controller class.
 */

class controller
{
    private $requestPost;
    private $requestGet;

    /**
     * Intialize with our existing array of todo items.
     *
     * @param array $list
     */
    public function __construct(array $post = [], array $get = [])
    {
        $this->requestPost = $post;
        $this->requestGet = $get;
    }

    /**
     * Route: / or /index
     *
     * @return void
     */
    public function index(){
        return 'page';
    }

    /**
     * Route: /add
     *
     * @param todo $todo
     * @return void
     */
    public function add(todo &$todo){
        if (isset($this->requestPost['item'])) {
            $todo->addItem($this->requestPost['item']);
        }
        return 'page';
    }

    /**
     * Route: /done
     *
     * @param todo $todo
     * @return void
     */
    public function done(todo &$todo){
        if (isset($this->requestPost['item'])) {
            $todo->setItemDone($this->requestPost['item']);
        }
        return 'page';
    }

    /**
     * Route: /notdone
     *
     * @param todo $todo
     * @return void
     */
    public function notdone(todo &$todo){
        if (isset($this->requestPost['item'])) {
            $todo->setItemNotDone($this->requestPost['item']);
        }
        return 'page';
    }

    /**
     * Route: /delete
     *
     * @param todo $todo
     * @return string
     */
    public function delete(todo &$todo){
        if (isset($this->requestPost['item'])) {
            $todo->delete($this->requestPost['item']);
        }
        return 'page';
    }
}
