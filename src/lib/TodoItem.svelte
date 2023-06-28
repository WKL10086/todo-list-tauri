<script lang="ts">
  import type { Todo } from "src/type/todo.type";

  export let item: Todo;

  let id = item.id.toString();
</script>

<div>
  <h2>{item.title}</h2>
  <label for={id} />
  <input type="checkbox" {id} name={id} value={item.completed} />
</div>
