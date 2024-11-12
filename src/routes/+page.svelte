<script lang="ts">
	import ChatSection from "./ChatSection.svelte";
	import { invoke } from "@tauri-apps/api/core";
	import { Input } from "$lib/components/ui/input";
	import ollama, { type Message } from "ollama/browser";
	import Button from "$lib/components/ui/button/button.svelte";
	import { Cog, PanelTopClose } from "lucide-svelte";
	import { blur } from "svelte/transition";

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

	function close() {
		invoke("hide_window");
	}

	function settings() {
		console.log("Settings");
	}

	function math_eval({ math }: { math: string }) {
		try {
			return "the result is: " + eval(math);
		} catch (e) {
			return "Invalid math expression";
		}
	}

	async function resp() {
		messages = [];
		isResponding = true;
		console.log("User message:", userMessage);
		const message = { role: "user", content: userMessage };
		messages = [...messages, message];
		const response = await ollama.chat({
			model: "llama3.2",
			messages,
			tools: [
				{
					type: "function",
					function: {
						name: "math_eval",
						description: "evaluates a simple math expression",
						parameters: {
							type: "object",
							properties: {
								math: {
									type: "string",
									description: "the math expression to evaluate",
								},
							},
							required: ["math"],
						},
					},
				},
				{
					type: "function",
					function: {
						name: "skip",
						description: "skips the tool-call because no tool is applicable",
						parameters: {
							type: "object",
							properties: {},
							required: [],
						},
					},
				},
			],
		});

		// Check if the model decided to use the provided function
		if (!response.message.tool_calls || response.message.tool_calls.length === 0) {
			console.log("The model didn't use the function. Its response was:");
			console.log(response.message.content);
		} else {
			console.log("The model used the function. Its response was:");
			console.log(response.message.tool_calls);
			console.log(response.message.content);
			if (response.message.tool_calls[0].function.name === "skip") {
				console.log("The model decided to skip the tool");
			} else {
				console.log("The model decided to use the tool");
				messages = [...messages, response.message];
			}
		}

		// Process function calls made by the model
		if (response.message.tool_calls) {
			const availableFunctions: { [key: string]: (args: any) => any } = {
				math_eval: math_eval,
				skip: () => "No tool is applicable",
			};
			for (const tool of response.message.tool_calls) {
				//if the tool is skip, we skip the tool
				if (tool.function.name === "skip") {
					continue;
				}

				const functionToCall = availableFunctions[tool.function.name];
				const functionResponse = functionToCall(tool.function.arguments);

				// Add function response to the conversation
				messages.push({
					role: "tool",
					content: functionResponse,
				});
			}
		}

		const finalResponse = await ollama.chat({
			model: "llama3.2",
			messages,
			stream: true,
		});

		messages = [...messages, { role: "assistant", content: "" }];
		for await (const part of finalResponse) {
			messages[messages.length - 1].content += part.message.content;
		}

		isResponding = false;
		const inputElement = document.getElementById("inputQueryElement") as HTMLInputElement;
		if (inputElement) {
			inputElement.disabled = false;
		} // force the input element to be enabled
		document.getElementById("inputQueryElement")?.focus();
	}
</script>

<div class="p-1 bg-background backdrop-blur-sm rounded-lg flex justify-between gap-1 shadow-lg">
	<div class="w-full">
		<Input
			id="inputQueryElement"
			placeholder="Ask Clippy..."
			bind:value={userMessage}
			disabled={isResponding}
			onkeypress={handleKeypress} />
	</div>
	{#if !userMessage}
		<div class="gap-1 backdrop-blur-sm rounded-lg flex flex-row justify-end">
			<Button onclick={settings} size="icon" variant="outline">
				<Cog />
			</Button>
			<Button onclick={close} size="icon" variant="outline" disabled={isResponding}>
				<PanelTopClose />
			</Button>
		</div>
	{/if}
</div>

{#if messages.length > 0}
	<div class="p-2 bg-background backdrop-blur-sm rounded-lg mt-4 flex flex-col gap-4 shadow-lg" in:blur>
		{#each messages as message}
			{#if message.role !== "user"}
				<ChatSection {message} />
			{/if}
		{/each}
	</div>
{/if}
