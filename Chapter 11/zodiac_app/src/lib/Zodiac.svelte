<script>
    import { invoke } from "@tauri-apps/api/tauri"
    let zodiac_msg = [""];
    let zodiac = ""
    async function handle_zodiac(){
      zodiac_msg = await invoke("get_zodiac", {zodiac})
    }
  </script>

  <div>
    <div class="row">
      <input bind:value={zodiac} placeholder="Libra...">
    </div>
    <button on:click={handle_zodiac}>Enter</button>

    {#each zodiac_msg as z}
    <p>{z}</p>
    {/each}
  </div>

