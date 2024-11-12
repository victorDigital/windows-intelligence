<script lang="ts">
	import ChatSection from "./ChatSection.svelte";

	import { invoke } from "@tauri-apps/api/core";
	import { Input } from "$lib/components/ui/input";
	import ollama, { type Message } from "ollama/browser";
	import type { SvelteComponent } from "svelte";
	import type { HTMLInputAttributes } from "svelte/elements";

	let messages: Message[] = $state([]);

	let userMessage = $state("");

	let isResponding = $state(false);

	function handleKeypress(event: KeyboardEvent) {
		if (event.key === "Enter") {
			event.preventDefault();
			userMessage = userMessage.trim();
			resp();
			userMessage = "";
		}
	}

	async function resp() {
		messages = [];
		isResponding = true;
		console.log("User message:", userMessage);
		const message = { role: "user", content: userMessage };
		messages = [...messages, message];
		const response = await ollama.chat({ model: "llama3.2", messages, stream: true });
		messages = [...messages, { role: "bot", content: "" }];
		for await (const part of response) {
			messages[messages.length - 1].content += part.message.content;
			console.log(part);
		}
		isResponding = false;
		const inputElement = document.getElementById("inputQueryElement") as HTMLInputElement;
		if (inputElement) {
			inputElement.disabled = false;
		} // force the input element to be enabled
		document.getElementById("inputQueryElement")?.focus();
	}
</script>

<div class="p-1 bg-background backdrop-blur-sm rounded-lg">
	<Input
		id="inputQueryElement"
		placeholder="Ask Clippy..."
		bind:value={userMessage}
		disabled={isResponding}
		onkeypress={handleKeypress} />
</div>

{#if messages.length > 0}
	<div class="p-2 bg-background backdrop-blur-sm rounded-lg mt-4 flex flex-col gap-4">
		{#each messages as message}
			{#if message.role === "bot"}
				<ChatSection {message} />
			{/if}
		{/each}
	</div>
{/if}
