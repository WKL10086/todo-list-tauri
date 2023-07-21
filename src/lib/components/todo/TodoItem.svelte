<script lang="ts">
  import type { Todo } from "../../../types/todo.type";

  export let item: Todo;

  let id = item.id;
</script>

<div class="grid">
  <label for={id}>
    <h2>
      {item.title}
    </h2>
  </label>
  <input type="checkbox" {id} name={id} value={item.completed} />
</div>
