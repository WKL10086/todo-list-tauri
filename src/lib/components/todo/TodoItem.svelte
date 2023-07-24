<script lang="ts">
  import type { Todo } from "../../../types/todo.type";

  export let item: Todo;

  let id = item.id;
</script>

<div class="grid">
  <label for={id}>
    <input type="checkbox" {id} name={id} checked={item.completed} />
    <strong>
      {item.title}
    </strong>
  </label>
</div>
