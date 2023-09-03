import { error } from '@sveltejs/kit';

export async function load({ params }) {
	const mediaList = await fetch('http://127.0.0.1:8001/api/media')
		.then((response) => response.json())
		.then((data) => {
			return data.media;
		})
		.catch((err) => {
			console.log({ err });
			return null;
		});

	if (mediaList) {
		return {
			mediaList
		};
	}

	throw error(404, 'Media not found');
}
