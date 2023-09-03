<script>
	import { onMount } from 'svelte';
	import {
		getMediaList,
		getMediaById,
		createMedia,
		updateMedia,
		deleteMedia,
		generateRandomMedia
	} from '$lib';

	import MovieCard from '../components/MovieCard.svelte';
	import ShowCard from '../components/ShowCard.svelte';

	import AddMediaModal from '../components/AddModal.svelte';
	import EditModal from '../components/EditModal.svelte';
	import DeleteModal from '../components/DeleteModal.svelte';
	import EmptyBox from '../components/EmptyBox.svelte';

	import { siteContent } from '$lib/data.js';

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

	let isGenerateMediaInputVisible = false;
	let generateMediaCount = 2;

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
		return [...new Set(mediaList?.map((media) => media.genres).flat())];
	}

	let genreOptions = extractUniqueGenres(data?.mediaList);

	let tabOptions = [
		{ label: 'All', value: 'all' },
		{ label: 'Movies', value: 'movie' },
		{ label: 'Shows', value: 'show' }
	];

	let sortOptions = Object.keys(data?.mediaList?.at(0) || {});

	const unwantedKeys = ['_id', 'description', 'genres'];
	sortOptions = sortOptions.filter((key) => !unwantedKeys.includes(key));

	async function fetchMediaList() {
		try {
			const response = await getMediaList({ limit: 50 });
			data.mediaList = response.media || [];
		} catch (error) {
			console.error(error.message);
		}
	}

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

		isGenerateMediaInputVisible = false;
		generateMediaCount = 2;
	}

	async function saveChanges(event) {
		console.log({ event });
		mediaItem = event.detail;
		console.log(`Saving ${mediaItem.title} - ${mediaItem._id}...`);
		isEditModalOpen = false;
		try {
			const updatedMedia = await updateMedia(mediaItem._id, mediaItem);
			console.log('Updated media:', updatedMedia);
		} catch (error) {
			console.error(error.message);
		}
	}

	async function deleteMediaItem(event) {
		console.log({ event });
		mediaItem = event.detail;
		console.log(`Deleting ${mediaItem.title} - ${mediaItem._id}...`);
		try {
			const response = await deleteMedia(mediaItem._id);
			console.log('Deleted media:', response);
		} catch (error) {
			console.error(error.message);
		}
	}

	async function addMedia(event) {
		console.log({ event });
		mediaItem = event.detail;
		console.log(`Adding ${mediaItem.title} - ${mediaItem._id}...`);
		try {
			const response = await createMedia(mediaItem);
			console.log('Created media:', response);
		} catch (error) {
			console.error(error.message);
		}
	}

	async function handleSubmit() {
		console.log('Generating random media count:', generateMediaCount);
		isGenerateMediaInputVisible = false;
		try {
			const response = await generateRandomMedia(generateMediaCount);
			console.log('Generated random media:', response);
		} catch (error) {
			console.error(error.message);
		}
	}

	function generateRandomMediaToggle() {
		isGenerateMediaInputVisible = true;
	}
</script>

<nav class="bg-gray-900 text-white p-4 sticky top-0">
	<div class="container mx-auto flex justify-between items-center">
		<h1
			class="text-2xl font-semibold border-b-2 border-white pb-2 hover:border-blue-500 transition duration-300 ease-in-out"
		>
			🚀 Media List
		</h1>
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
</nav>

<main class="container mx-auto p-4">
	<div class="mb-4">
		<h1 class="text-2xl font-semibold mb-4">How This Site Works</h1>
		<p class="text-gray-700 mb-4">Welcome to the media list website! Here's how it works:</p>
		<ul class="list-disc ml-6">
			{#each siteContent.howThisSiteWorks as item}
				<li class="mb-2">
					<span class="text-blue-500 font-semibold">{item.title}:</span>
					{item.description}
				</li>
			{/each}
		</ul>
	</div>

	<div class="mb-8">
		<h1 class="text-2xl font-semibold mb-4">Backend Endpoints (Rust)</h1>
		<p class="text-gray-700 mb-4">
			Our site's backend, powered by the Rust programming language, offers a set of API endpoints to
			ensure seamless data retrieval, manipulation, and management. Here's an overview of the
			essential endpoints:
		</p>
		<ul class="list-disc ml-6">
			{#each siteContent.backendEndpoints as item}
				<li class="mb-2">
					<span class="text-blue-500 font-semibold">{item.title}:</span>
					{item.description}
				</li>
			{/each}
		</ul>
	</div>

	<div class="mb-8">
		<h1 class="text-2xl font-semibold mb-4">Frontend with SvelteKit</h1>
		<p class="text-gray-700 mb-4">
			Our site's frontend is built using SvelteKit, a dynamic JavaScript framework that powers the
			user interface and interactions. Here's how SvelteKit enhances your experience:
		</p>
		<ul class="list-disc ml-6">
			{#each siteContent.frontendSvelteKit as item}
				<li class="mb-2">
					<span class="text-blue-500 font-semibold">{item.title}:</span>
					{item.description}
				</li>
			{/each}
		</ul>
	</div>

	<div class="flex justify-between items-center mb-4">
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
		<div class="flex space-x-4">
			{#if isGenerateMediaInputVisible}
				<form on:submit={handleSubmit}>
					<input
						type="number"
						min="2"
						max="40"
						bind:value={generateMediaCount}
						class="border rounded p-2 w-16"
					/>
					<button
						type="submit"
						class="bg-yellow-500 text-white p-2 rounded-md hover:bg-yellow-600 transition duration-300 ease-in-out"
					>
						Generate
					</button>
				</form>
			{:else}
				<button
					class="bg-yellow-500 text-white p-2 rounded-md hover:bg-yellow-600 transition duration-300 ease-in-out"
					on:click={generateRandomMediaToggle}
				>
					Generate Random Media
				</button>
			{/if}
			<button
				class="bg-green-500 text-white p-2 rounded-md hover:bg-green-600 transition duration-300 ease-in-out"
				on:click={() => (isAddModalOpen = true)}
			>
				Add Media
			</button>
		</div>
	</div>

	{#if data.mediaList.length == 0}
		<EmptyBox />
	{/if}
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
