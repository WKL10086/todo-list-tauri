<script lang="ts">
  // it would affect the whole app no matter where it is imported
  import "@picocss/pico";

  import TodoList from "./lib/components/todo/TodoList.svelte";
</script>

<main class="container">
  <h1>Todo List !</h1>

  <div class="row">
    <TodoList />
  </div>
</main>
