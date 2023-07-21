<script lang="ts">
  // it would affect the whole app no matter where it is imported
  import "@picocss/pico";

  import Layout from "./lib/components/layout/Layout.svelte";
  import TodoList from "./lib/components/todo/TodoList.svelte";
</script>

<Layout>
  <TodoList />
</Layout>
