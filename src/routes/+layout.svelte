<script lang="ts">
	import "../app.css";
	import { ModeWatcher } from "mode-watcher";
	let { children } = $props();
</script>

<ModeWatcher />
<main class="h-screen w-full max-h-screen min-h-screen min-w-full max-w-full p-2 bg-background/25 rounded-xl">
	{@render children?.()}
</main>
