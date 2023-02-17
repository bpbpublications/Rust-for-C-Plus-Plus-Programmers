<script>
  import Zodiac from "$lib/Zodiac.svelte";
</script>

<h1>Welcome to Programming Zodiacs!</h1>

<div class="row">
  <img src="/ProgrammingIllustration.png" height="200px">
</div>

<p>Please enter your zodiac sign... </p>

<div class="row">
  <Zodiac/>
</div>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>
