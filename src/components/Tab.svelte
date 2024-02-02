<script lang="ts">
    export let name: string;
    export let open: boolean = false;
</script>
<button
class="hover:bg-[#e6497d] {open ? "bg-#e6497d" : "bg-red-700"} text-white font-bold py-0.5 px-4 rounded-tl-lg rounded-tr-lg min-w-40 border-l-2 border-t-2 border-r-2 border-pink-800"
on:click
on:auxclick
on:contextmenu>
    {name}
</button>