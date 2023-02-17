<script>
  import Rss from "$lib/Rss.svelte";
</script>

<h1>Welcome to RSS News App!</h1>

<p>Press the refresh button for the latest stories</p>

<div class="row">
  <Rss/>
</div>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>
