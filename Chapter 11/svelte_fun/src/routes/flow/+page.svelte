<script lang="ts">
    let counter: number = 1;
    function double(){
        counter *= 2;
    }
    function reset(){
        counter = 1;
    }
</script>

{#if counter < 10}
    <h1>Welcome to Double the Counter!</h1>
    <p>Keep doubling the count!!!</p>
{:else if counter >= 10}
    <h1>Woah the count is high!!!!</h1>
    <p>RESET!!!!!!!!</p>
{/if}


<p>Count: {counter}</p>

<button on:click={double}>Double Counter</button>
<br>
<button on:click={reset}>Reset Counter</button>

<style>

</style>