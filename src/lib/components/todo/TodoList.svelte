<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import TodoItem from "./TodoItem.svelte";
  import { todoList } from "./store";
  import type { Todo } from "../../../types/todo.type";

  let todoListValue;

  todoList.subscribe((value) => {
    todoListValue = value;
  });

  async function getTodoList() {
    let result: Todo[] = await invoke("get_init_todo_list");
    $todoList = result;
  }

  getTodoList();
</script>

<div>
  <h1>Todo List</h1>
  {#each todoListValue as item (item.id)}
    <TodoItem {item} />
  {/each}
</div>
