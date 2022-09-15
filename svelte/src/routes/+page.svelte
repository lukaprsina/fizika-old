<script lang="ts">
	import Editor from '@tinymce/tinymce-svelte';
	import test from '$lib/testpage';
	let value = test;
</script>

<main>
	<Editor bind:value />
	<div>{value}</div>
	<textarea bind:value />
</main>
