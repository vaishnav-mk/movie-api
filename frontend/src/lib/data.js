export const siteContent = {
	howThisSiteWorks: [
		{
			title: 'View Media',
			description: 'You can browse our collection of movies and shows on the Media List page.'
		},
		{
			title: 'Filter and Sort',
			description:
				'Use the filters and sorting options to find specific media based on genres, titles, and more.'
		},
		{
			title: 'Add Media',
			description:
				"Want to add your own favorite movies or shows? Click the 'Add Media' button and fill in the details."
		},
		{
			title: 'Edit and Delete',
			description:
				"If you need to make changes to your entries, use the 'Edit' and 'Delete' options."
		},
		{
			title: 'Generate Random Media',
			description:
				"Feeling adventurous? Click 'Generate Random Media' to randomly generate new titles."
		}
	],
	backendEndpoints: [
		{
			title: 'Media List Endpoint',
			description:
				'The /api/media endpoint allows you to retrieve a list of movies and shows from our database.'
		},
		{
			title: 'Media Details Endpoint',
			description:
				'Using the /api/media/:id endpoint, you can access detailed information about a specific media item by providing its unique ID.'
		},
		{
			title: 'Add Media Endpoint',
			description:
				'Our /api/media POST endpoint facilitates the addition of new media items, ensuring they are seamlessly integrated into our collection.'
		},
		{
			title: 'Edit Media Endpoint',
			description:
				'With the /api/media/:id PATCH endpoint, you can efficiently update existing media entries, including title, description, genres, rating, and status.'
		},
		{
			title: 'Delete Media Endpoint',
			description:
				'The /api/media/:id DELETE endpoint enables the removal of media items from our database.'
		},
		{
			title: 'Random Media Generator Endpoint',
			description:
				'For some fun and inspiration, the /api/generate-media/:n endpoint generates random movie and show suggestions.'
		}
	],
	frontendSvelteKit: [
		{
			title: 'User-Friendly Interface',
			description:
				'SvelteKit ensures a user-friendly and responsive interface, making it easy to navigate and interact with media items.'
		},
		{
			title: 'Efficiency and Performance',
			description:
				"SvelteKit's efficient code and data-binding approach result in a high-performance web application, ensuring a smooth and enjoyable browsing experience."
		}
	]
};
