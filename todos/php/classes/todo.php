<?php
namespace Todo\PHP;

/* A minimum viable ToDo application in PHP.
 * No user sessions or saving.
 */

class todo
{
    private $items;

    /**
     * Intialize with our existing array of todo items.
     *
     * @param array $list
     */
    public function __construct(array $list = [])
    {
        $this->items = $list;
    }

    /**
     * Add a new todo item, default to not done.
     *
     * @param string $name
     * @return void
     */
    public function addItem(string $name)
    {
        $this->items[] = array('name' => $name, 'done' => 0);
    }

    /**
     * Delete an item at the given number.
     *
     * @param int $num
     * @return void
     */
    public function delete(int $num)
    {
        unset($this->items[$num]);
    }

    /**
     * Fetch our current list of todo items as an array.
     *
     * @return array
     */
    public function getItems(): array
    {
        return $this->items;
    }

    /**
     * Get our array of current items as a JSON string.
     * For passing to Javascript or persistence in hidden form fields.
     *
     * @return string
     */
    public function getItemsJSON(): string
    {
        return json_encode($this->items);
    }

    /**
     * Mark a todo item as complete.
     *
     * @param integer $num
     * @return void
     */
    public function setItemDone(int $num)
    {
        $this->items[$num]['done'] = 1;
    }

    /**
     * Mark a todo item as incomplete.
     *
     * @param integer $num
     * @return void
     */
    public function setItemNotDone(int $num)
    {
        $this->items[$num]['done'] = 0;
    }
}
