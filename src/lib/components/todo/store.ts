import { Todo } from "src/types/todo.type";
import { writable } from "svelte/store";

export const todoList = writable<Todo[]>([]);
