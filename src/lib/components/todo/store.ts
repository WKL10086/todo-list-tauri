import type { Todo } from "../../../types/todo.type";
import { writable } from "svelte/store";

export const todoList = writable<Todo[]>([
  {
    id: "0",
    title: "Default todo",
    completed: false,
  },
]);
