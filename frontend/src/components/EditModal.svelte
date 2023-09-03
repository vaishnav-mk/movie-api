<script>
	import { createEventDispatcher } from 'svelte';

	export let mediaItem;
	export let isEditModalOpen;

	const dispatch = createEventDispatcher();

	function openModal() {
		isEditModalOpen = true;
	}

	function closeModal() {
		isEditModalOpen = false;
	}

	function saveChanges() {
		dispatch('save', mediaItem);
		closeModal();
	}
</script>

{#if isEditModalOpen && mediaItem}
	<div class="fixed top-0 left-0 w-full h-full flex items-center justify-center z-50">
		<div class="bg-white rounded-lg p-6 shadow-md w-96">
			<h2 class="text-lg font-semibold mb-4">Edit Media</h2>

			<input
				type="text"
				bind:value={mediaItem.title}
				placeholder="Title"
				class="mb-2 border p-2 rounded-md w-full"
			/>
			<textarea
				bind:value={mediaItem.description}
				placeholder="Description"
				rows="4"
				class="mb-2 border p-2 rounded-md w-full"
			/>

			<input
				type="text"
				bind:value={mediaItem.genres}
				placeholder="Genres (comma-separated)"
				class="mb-2 border p-2 rounded-md w-full"
			/>
			<div class="mb-2">
				<label class="block mb-1 text-sm">Rating:</label>
				<div class="flex space-x-2">
					{#each [1, 2, 3, 4, 5] as starValue}
						<span
							class={`rating-star ${
								starValue <= mediaItem.rating ? 'text-yellow-500' : 'text-gray-400'
							}`}
							on:click={() => (mediaItem.rating = starValue)}
						>
							&#9733;
						</span>
					{/each}
				</div>
			</div>

			<div class="mb-2">
				<label class="block mb-1 text-sm">Type:</label>
				<div class="flex space-x-2">
					<button
						class={`px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 transition duration-300 ease-in-out ${
							mediaItem.type === 'Movie' ? 'bg-blue-600' : ''
						}`}
						on:click={() => (mediaItem.type = 'Movie')}
					>
						Movie
					</button>
					<button
						class={`px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 transition duration-300 ease-in-out ${
							mediaItem.type === 'Show' ? 'bg-blue-600' : ''
						}`}
						on:click={() => (mediaItem.type = 'Show')}
					>
						Show
					</button>
				</div>
			</div>

			<div class="mb-2">
				<label class="block mb-1 text-sm">Status:</label>
				<select bind:value={mediaItem.status} class="mb-2 border p-2 rounded-md w-full">
					{#each [
						{ label: 'Watching', value: 'Watching' },
						{ label: 'Watched', value: 'Watched' },
						{ label: 'Dropped', value: 'Dropped' },
						{ label: 'On Hold', value: 'OnHold' },
						{ label: 'Plan to watch', value: 'PlanToWatch' }
					] as status}
						<option value={status.value}>
							{mediaItem.status === status.value ? `Selected: ${status.label}` : status.label}
						</option>
					{/each}
				</select>
			</div>

			<div class="flex justify-end">
				<button
					class="px-4 py-2 text-blue-500 border border-blue-500 rounded-md mr-2"
					on:click={closeModal}
				>
					Cancel
				</button>
				<button class="px-4 py-2 bg-blue-500 text-white rounded-md" on:click={saveChanges}>
					Save
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.rating-star {
		@apply inline-block text-2xl cursor-pointer;
	}
</style>
