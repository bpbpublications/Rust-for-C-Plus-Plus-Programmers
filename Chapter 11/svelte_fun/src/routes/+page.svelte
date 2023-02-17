<script lang="ts">
    let counter: number = 0;
    function add_counter(){
        counter++;
    }
    function min_counter(){
        counter--;
    }
    export function get_counter(){
        return counter;
    }
    
</script>

<!--HTML-->
<p>Count: {counter}</p>

<button on:click={add_counter}>Add Counter</button>
<button on:click={min_counter}>Decrease Counter</button>

<!-- 
Another way of doing it is: 
<button on:click={() => counter++}>Add Counter</button>
<button on:click={() => counter--}>Decrease Counter</button>
-->
<br>
<a href="/flow">Go to Flow</a>

<style>


</style>