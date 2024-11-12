<script lang="ts">
	import Separator from "$lib/components/ui/separator/separator.svelte";
	import type { Message } from "ollama/browser";
	import { Paperclip } from "lucide-svelte";
	import { blur, fade } from "svelte/transition";

	let { message }: { message: Message } = $props();
</script>

<section class="w-full" in:blur>
	{#if message.role === "assistant"}
		<div class="flex items-center gap-1 text-foreground/60">
			<Paperclip class="size-4" />
			<span class="text-sm">Clippy</span>
		</div>
		{#if message.content}
			<p in:blur>{message.content}</p>
		{:else if (message.tool_calls ?? []).length > 0}
			<p>Using tool</p>
		{:else}
			<p>loading...</p>
		{/if}
	{:else if message.role === "tool"}
		<p class="font-mono text-sm">used tool</p>
		<p class="font-mono text-sm text-foreground/50">{message.content}</p>
	{/if}
</section>
