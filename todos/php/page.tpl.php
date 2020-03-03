<!DOCTYPE html>
<html>
  <head>
    <title>Todo MVP</title>
    <meta charset="utf-8">
    <link rel="stylesheet" href="/static/todo.css">
  </head>
  <body>
    <h1>Todo MVP</h1>
    <section class="new-todo">
      <form method="POST" action="/">
        <input type="text"
               id="new-item"
               name="item"
               pattern=".{3,}"
               required="required"
               aria-label="Write a new todo item"
               title="3 characters minimum" />
        <input type="hidden"
               name="current-list"
               value="<?=$currentList;?>" />
        <input type="submit"
               value="Add new item"
               id="add-new-item"
               name="add-new-item" />
      </form>
    </section>

    <section class="items">
      <h2>Todo list</h2>
      <ul><?php
      foreach($todos as $num => $todo){
        if ($todo['done']){ ?>
        <li class="todo done">
          <span class="item-name">
            <s><?=$todo['name'];?></s>
          </span>
          <form method="post" action="/notdone">
            <input type="hidden" name="item" value="<?=$num;?>"/>
            <input type="hidden"
               name="current-list"
               value="<?=$currentList;?>" />
            <input class="uncomplete"
                   type="submit"
                   name="mark-not-done"
                   value="Mark not done '<?=$todo['name'];?>'" />
          </form>
          <form method="post" action="/delete">
            <input type="hidden" name="item" value="<?=$num;?>"/>
            <input type="hidden"
               name="current-list"
               value="<?=$currentList;?>" />
            <input class="delete"
                   type="submit"
                   name="delete"
                   value="Delete '<?=$todo['name'];?>'" />
          </form>
        </li>
  <?php }else { ?>
        <li class="todo">
          <span class="item-name">
          <?=$todo['name'];?>
          </span>
          <form method="post" action="/done">
            <input type="hidden" name="item" value="<?=$num;?>"/>
            <input type="hidden"
               name="current-list"
               value="<?=$currentList;?>" />
            <input class="complete"
                   type="submit"
                   name="mark-done"
                   value="Mark done '<?=$todo['name'];?>'" />
          </form>
          <form method="post" action="/delete">
            <input type="hidden" name="item" value="<?=$num;?>"/>
            <input type="hidden"
               name="current-list"
               value="<?=$currentList;?>" />
            <input class="delete"
                   type="submit"
                   name="delete"
                   value="Delete '<?=$todo['name'];?>'" />
          </form>
        </li>
        <?php }
    } ?></ul>
    </section>
  </body>
</html>
