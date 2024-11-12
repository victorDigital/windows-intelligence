<script lang="ts">
	import Separator from "$lib/components/ui/separator/separator.svelte";
	import type { Message } from "ollama/browser";
	import { Paperclip } from "lucide-svelte";

	let { message }: { message: Message } = $props();
</script>

<section class="w-full">
	{#if message.role === "bot"}
		<div class="flex items-center gap-1 text-foreground/60">
			<Paperclip class="size-4" />
			<span class="text-sm">Clippy</span>
		</div>
	{:else if message.role === "user"}
		<div class="flex items-center gap-1 text-foreground/60">
			<Paperclip class="size-4" />
			<span class="text-sm">You</span>
		</div>
	{/if}
	<p>{message.content}</p>
</section>
