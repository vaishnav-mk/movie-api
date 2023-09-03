<script>
	import { createEventDispatcher } from 'svelte';

	export let isAddModalOpen;

	const dispatch = createEventDispatcher();

	let newMedia = {
		title: '',
		description: '',
		genres: [],
		rating: 3,
		type: 'Movie',
		status: 'Watching'
	};

	let selectedRating = 3;

	function closeModal() {
		dispatch('closeModal');
	}

	function saveMedia() {
		newMedia.rating = selectedRating;

		newMedia.genres = newMedia.genres
			.split(',')
			.map((genre) => genre.trim())
			.filter((genre) => genre !== '');

		dispatch('add', newMedia);
		closeModal();

		newMedia = {
			title: '',
			description: '',
			genres: [],
			rating: 3,
			type: 'Movie',
			status: 'Watching'
		};
		selectedRating = 3;
	}
</script>

{#if isAddModalOpen}
	<div class="fixed top-0 left-0 w-full h-full flex items-center justify-center z-50">
		<div class="bg-white rounded-lg p-6 shadow-md w-96">
			<h2 class="text-lg font-semibold mb-4">Add Media</h2>

			<input
				type="text"
				bind:value={newMedia.title}
				placeholder="Title"
				class="mb-2 border p-2 rounded-md w-full"
			/>
			<textarea
				bind:value={newMedia.description}
				placeholder="Description"
				rows="4"
				class="mb-2 border p-2 rounded-md w-full"
			/>

			<input
				type="text"
				bind:value={newMedia.genres}
				placeholder="Genres (comma-separated)"
				class="mb-2 border p-2 rounded-md w-full"
			/>
			<div class="mb-2">
				<label class="block mb-1 text-sm">Rating:</label>
				<div class="flex space-x-2">
					{#each [1, 2, 3, 4, 5] as starValue}
						<span
							class={`rating-stars ${
								starValue <= selectedRating ? 'text-yellow-500' : 'text-gray-400'
							}`}
							on:click={() => (selectedRating = starValue)}
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
						class={`px-4 py-2 bg-blue-500 text-white rounded-md transition duration-300 ease-in-out ${
							newMedia.type === 'Movie' ? 'bg-green-600' : 'hover:bg-blue-600'
						}`}
						on:click={() => (newMedia.type = 'Movie')}
					>
						Movie
					</button>
					<button
						class={`px-4 py-2 bg-blue-500 text-white rounded-md transition duration-300 ease-in-out ${
							newMedia.type === 'Show' ? 'bg-green-600' : 'hover:bg-blue-600'
						}`}
						on:click={() => (newMedia.type = 'Show')}
					>
						Show
					</button>
				</div>
			</div>

			<div class="mb-2">
				<label class="block mb-1 text-sm">Status:</label>
				<select bind:value={newMedia.status} class="mb-2 border p-2 rounded-md w-full">
					{#each ['Watching', 'Watched', 'Dropped', 'OnHold', 'PlanToWatch'] as status}
						<option value={status}>
							{newMedia.status === status ? `Selected: ${status}` : status}
						</option>
					{/each}
				</select>
			</div>

			<div class="flex justify-end gap-2">
				<button class="px-4 py-2 bg-blue-500 text-white rounded-md" on:click={saveMedia}>
					Save
				</button>
				<button
					class="px-4 py-2 text-blue-500 border border-blue-500 rounded-md"
					on:click={closeModal}
				>
					Cancel
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.rating-stars {
		@apply inline-block text-2xl cursor-pointer;
	}
</style>
