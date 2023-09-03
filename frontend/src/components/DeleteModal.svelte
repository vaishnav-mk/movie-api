<script>
	import { createEventDispatcher } from 'svelte';

	export let mediaItem;
	export let isDeleteModalOpen;

	const dispatch = createEventDispatcher();

	function closeModal() {
		dispatch('closeModal');
	}

	function deleteItem() {
		dispatch('delete', mediaItem);
		closeModal();
	}
</script>

{#if isDeleteModalOpen}
	<div class="fixed top-0 left-0 w-full h-full flex items-center justify-center z-50">
		<div class="bg-white rounded-lg p-6 shadow-md w-96">
			<h2 class="text-lg font-semibold mb-4">Delete Media</h2>
			<p class="mb-4">Are you sure you want to delete the media item "{mediaItem.title}"?</p>

			<div class="flex justify-end gap-2">
				<button class="px-4 py-2 bg-red-500 text-white rounded-md" on:click={deleteItem}>
					Delete
				</button>
				<button
					class="px-4 py-2 text-gray-500 border border-gray-500 rounded-md"
					on:click={closeModal}>Cancel</button
				>
			</div>
		</div>
	</div>
{/if}
