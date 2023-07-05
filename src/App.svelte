<script lang="ts">
  import TodoList from "./lib/components/todo/TodoList.svelte";
  import "@picocss/pico";
</script>

<main class="container">
  <h1>Todo List !</h1>

  <div class="row">
    <TodoList />
    <form>
      <input type="text" placeholder="Text" aria-label="Text" />
      <input type="email" placeholder="Email" aria-label="Email" />
      <input type="number" placeholder="Number" aria-label="Number" />
      <input type="password" placeholder="Password" aria-label="Password" />
      <input type="tel" placeholder="Tel" aria-label="Tel" />
      <input type="url" placeholder="Url" aria-label="Url" />
    </form>
  </div>
</main>
