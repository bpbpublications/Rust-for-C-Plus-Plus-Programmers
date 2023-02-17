<script>
    import { invoke } from "@tauri-apps/api/tauri"
    let titles = [""]
    let dates = [""]
    let links = [""]

    async function get_feed(){
      titles = await invoke("get_titles")
      dates = await invoke("get_dates")
      links = await invoke("get_links")
    }
  </script>

<div>
    <div class="row">
      <button on:click={get_feed}>Refresh</button>  
    </div>
<br>
<div class="row">
  <table class="w3-table w3-bordered">
    <tr>
      <th>Title</th>
      <th>Publish Date</th>
    </tr>
    {#each titles as _, i}
      <tr>
        <a href="{links[i]}"><td>{titles[i]}</td></a>
        <td>{dates[i]}</td>
      </tr>
    {/each}
  </table>
</div>
</div>

