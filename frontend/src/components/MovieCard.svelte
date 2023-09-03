<!-- MovieCard.svelte -->
<script>
	import { fly, slide } from 'svelte/transition';
	import { createEventDispatcher } from 'svelte';

	export let movie;

	const dispatch = createEventDispatcher();

	function getStarIcons(rating) {
		const fullStars = Math.floor(rating);
		const halfStar = rating % 1 !== 0;
		const emptyStars = 5 - Math.ceil(rating);

		return {
			fullStars: Array(fullStars).fill('★'),
			halfStar: halfStar ? ['½'] : [],
			emptyStars: Array(emptyStars).fill('☆')
		};
	}

	function handleEditClick() {
		dispatch('edit', movie);
	}

	function handleDeleteClick() {
		dispatch('delete', movie);
	}
</script>

<div
	class="border-dashed border-blue-500 border rounded-lg shadow-lg overflow-hidden hover:shadow-xl transition duration-300 ease-in-out transform hover:-translate-y-1 hover:scale-1.02 bg-blue-100"
	in:slide={{ y: -200, duration: 1000 }}
	out:fly={{ y: 200, duration: 1000 }}
>
	<div class="p-4">
		<div class="flex justify-between items-center mb-4">
			<h2 class="text-lg font-semibold">{movie.title}</h2>
			<div class="flex items-center">
				{#each getStarIcons(movie.rating).fullStars as star}
					<span class="text-yellow-500 pr-1">{star}</span>
				{/each}
				{#each getStarIcons(movie.rating).halfStar as star}
					<span class="text-yellow-500 pr-1">{star}</span>
				{/each}
				{#each getStarIcons(movie.rating).emptyStars as star}
					<span class="text-gray-300 pr-1">{star}</span>
				{/each}
			</div>
		</div>
		<table class="w-full table-fixed mb-2">
			<tbody>
				<tr>
					<td class="w-1/4 text-gray-600 font-semibold">Description:</td>
					<td class="w-3/4 text-gray-600">{movie.description}</td>
				</tr>
				<tr>
					<td class="w-1/4 text-gray-600 font-semibold">Genres:</td>
					<td class="w-3/4 text-gray-600">{movie.genres.join(', ')}</td>
				</tr>
				<tr>
					<td class="w-1/4 text-gray-600 font-semibold">Status:</td>
					<td class="w-3/4 text-green-800">
						<span
							class="bg-green-200 px-2 py-1 rounded-md transition duration-300 ease-in-out hover:bg-green-300"
						>
							{movie.status}
						</span>
					</td>
				</tr>
				<tr>
					<td class="w-1/4 text-gray-600 font-semibold">Type:</td>
					<td class="w-3/4 text-gray-600">{movie.type}</td>
				</tr>
			</tbody>
		</table>
		<div class="mt-4">
			<button
				class="bg-blue-500 text-white p-2 rounded-md hover:bg-blue-600 transition duration-300 ease-in-out mr-2"
				on:click={handleEditClick}
			>
				✎ Edit
			</button>
			<button
				class="bg-red-500 text-white p-2 rounded-md hover:bg-red-600 transition duration-300 ease-in-out"
				on:click={handleDeleteClick}
			>
				🗑 Delete
			</button>
		</div>
	</div>
</div>
