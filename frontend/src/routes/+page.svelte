<script>
	import MovieCard from '../components/MovieCard.svelte';
	import ShowCard from '../components/ShowCard.svelte';

	import AddMediaModal from '../components/AddModal.svelte';
	import EditModal from '../components/EditModal.svelte';
	import DeleteModal from '../components/DeleteModal.svelte';

	export let data;
	let filter = '';
	let selectedTab = 'all';
	let selectedSort = 'type';
	let sortOrder = 1;
	let selectedGenre = '';

	let mediaItem = null;

	let isEditModalOpen = false;
	let isDeleteModalOpen = false;
	let isAddModalOpen = false;

	function sortByField(field) {
		if (field === selectedSort) {
			sortOrder = -sortOrder;
		} else {
			sortOrder = 1;
		}
		selectedSort = field;
	}

	function filterByGenre(genre) {
		selectedGenre = genre;
	}

	function extractUniqueGenres(mediaList) {
		return [...new Set(mediaList.map((media) => media.genres).flat())];
	}

	let genreOptions = extractUniqueGenres(data.mediaList);

	let tabOptions = [
		{ label: 'All', value: 'all' },
		{ label: 'Movies', value: 'movie' },
		{ label: 'Shows', value: 'show' }
	];

	let sortOptions = Object.keys(data.mediaList[0]);

	const unwantedKeys = ['_id', 'description', 'genres'];
	sortOptions = sortOptions.filter((key) => !unwantedKeys.includes(key));

	function handleEdit(item) {
		console.log({ item, isEditModalOpen });
		mediaItem = item.detail;
		isEditModalOpen = true;
	}

	function handleDelete(item) {
		console.log({ item, isDeleteModalOpen });
		mediaItem = item.detail;
		isDeleteModalOpen = true;
	}

	function closeModal() {
		console.log({ isEditModalOpen, isDeleteModalOpen });
		isEditModalOpen = false;
		isDeleteModalOpen = false;
		isAddModalOpen = false;
	}

	function saveChanges(event) {
		console.log({ event });
		mediaItem = event.detail;
		console.log(`Saving ${mediaItem.title} - ${mediaItem._id}...`);
		isEditModalOpen = false;
	}

	function deleteMediaItem(event) {
		console.log({ event });
		mediaItem = event.detail;
		console.log(`Deleting ${mediaItem.title} - ${mediaItem._id}...`);
	}

	function addMedia(event) {
		console.log({ event });
		mediaItem = event.detail;
		console.log(`Adding ${mediaItem.title} - ${mediaItem._id}...`);
	}
</script>

<div class="bg-gray-900 text-white p-4">
	<div class="container mx-auto flex justify-between items-center">
		<h1 class="text-2xl font-semibold">Media List</h1>
		<div class="flex space-x-4">
			<div class="relative">
				<input
					type="text"
					id="filter"
					class="border rounded-l p-2 focus:outline-none focus:border-blue-500 bg-gray-800 text-white"
					placeholder="Filter by Title"
					bind:value={filter}
				/>
				<button
					class="bg-blue-500 text-white p-2 rounded-r hover:bg-blue-600 transition duration-300 ease-in-out"
				>
					Search
				</button>
			</div>
			<select
				class="border rounded p-2 focus:outline-none focus:border-blue-500 bg-gray-800 text-white"
				bind:value={selectedSort}
				on:change={() => sortByField(selectedSort)}
			>
				{#each sortOptions as option}
					<option value={option}>Sort by {option}</option>
				{/each}
			</select>
			<button
				class="bg-blue-500 text-white p-2 rounded hover:bg-blue-600 transition duration-300 ease-in-out"
				on:click={() => (sortOrder *= -1)}
			>
				{sortOrder === 1 ? 'Ascending' : 'Descending'}
			</button>
			<select
				class="border rounded p-2 focus:outline-none focus:border-blue-500 bg-gray-800 text-white"
				bind:value={selectedGenre}
				on:change={() => filterByGenre(selectedGenre)}
			>
				<option value="">All Genres</option>
				{#each genreOptions as option}
					<option value={option}>{option}</option>
				{/each}
			</select>
		</div>
	</div>
</div>

<main class="container mx-auto mt-4 p-4">
	<div class="flex justify-between items-center">
		<h1 class="text-2xl font-semibold mb-2">Media List</h1>
		<div class="flex space-x-4">
			<button
				class="bg-yellow-500 text-white p-2 rounded-md hover:bg-yellow-600 transition duration-300 ease-in-out"
			>
				Generate Media
			</button>
			<button
				class="bg-green-500 text-white p-2 rounded-md hover:bg-green-600 transition duration-300 ease-in-out"
				on:click={() => (isAddModalOpen = true)}
			>
				Add Media
			</button>
		</div>
	</div>
	<div class="mb-4">
		<div class="flex">
			{#each tabOptions as tab}
				<div
					class={`mr-2 cursor-pointer px-4 py-2 rounded-lg text-blue-500 bg-blue-100 transition duration-300 ease-in-out font-bold ${
						selectedTab === tab.value ? 'bg-blue-500 text-white' : 'hover:bg-blue-200'
					}`}
					on:click={() => (selectedTab = tab.value)}
					aria-label={tab.label}
				>
					{tab.label}
				</div>
			{/each}
		</div>
	</div>

	<div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
		{#each data.mediaList
			.filter((media) => (selectedTab === 'all' || media.type.toLowerCase() === selectedTab) && (filter === '' || media.title.includes(filter)) && (selectedGenre === '' || media.genres.includes(selectedGenre)))
			.sort((a, b) => {
				if (selectedSort === 'rating') {
					return (a[selectedSort] - b[selectedSort]) * sortOrder;
				}
				return a[selectedSort].localeCompare(b[selectedSort]) * sortOrder;
			}) as media, index}
			{#if media.type === 'Movie'}
				<MovieCard movie={media} {index} on:edit={handleEdit} on:delete={handleDelete} />
			{:else if media.type === 'Show'}
				<ShowCard show={media} {index} on:edit={handleEdit} on:delete={handleDelete} />
			{/if}
		{/each}
	</div>

	<EditModal {mediaItem} {isEditModalOpen} on:closeModal={closeModal} on:save={saveChanges} />
	<DeleteModal
		{mediaItem}
		{isDeleteModalOpen}
		on:closeModal={closeModal}
		on:delete={deleteMediaItem}
	/>
	<AddMediaModal {isAddModalOpen} on:closeModal={closeModal} on:add={addMedia} />
</main>
