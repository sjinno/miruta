<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let name = "";
  let greetMsg = "";
  let url = "";

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    // greetMsg = await invoke("greet", { name });
    url = await invoke("query_anime", { query: "demon slayer" });
  }
</script>

<div class="container">
  <button on:click={greet}>boop</button>
  <p>Hello, world!</p>
  <p>{url}</p>
</div>
