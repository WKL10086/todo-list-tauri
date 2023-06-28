<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import TodoItem from "./TodoItem.svelte";
  import type { Todo } from "src/types/todo.type";

  let todoList: Todo[] = [];

  async function getTodoList() {
    todoList = await invoke("get_todo_list");
  }

  getTodoList();
</script>

<div>
  <h1>Todo List</h1>
  {#each todoList as item (item.id)}
    <TodoItem {item} />
  {/each}
</div>
