const BASE_URL = 'http://localhost:8080/api';

export async function getMediaList(queryOptions) {
	const response = await fetch(`${BASE_URL}/media${getQueryString(queryOptions)}`);
	if (!response.ok) {
		throw new Error('Failed to fetch media list');
	}
	return response.json();
}

export async function getMediaById(id) {
	const response = await fetch(`${BASE_URL}/media/${id}`);
	if (!response.ok) {
		throw new Error('Failed to fetch media by ID');
	}
	return response.json();
}

export async function createMedia(mediaData) {
	const response = await fetch(`${BASE_URL}/media`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(mediaData)
	});
	if (!response.ok) {
		throw new Error('Failed to create media');
	}
	return response.json();
}

export async function updateMedia(id, updateData) {
	const response = await fetch(`${BASE_URL}/media/${id}`, {
		method: 'PATCH',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(updateData)
	});
	if (!response.ok) {
		throw new Error('Failed to update media');
	}
	return response.json();
}

export async function deleteMedia(id) {
	const response = await fetch(`${BASE_URL}/media/${id}`, {
		method: 'DELETE'
	});
	if (!response.ok) {
		throw new Error('Failed to delete media');
	}
	return response.json();
}

export async function generateRandomMedia(n) {
	const response = await fetch(`${BASE_URL}/generate-media/${n}`);
	if (!response.ok) {
		throw new Error('Failed to generate random media');
	}
	return response.json();
}

function getQueryString(queryOptions) {
	if (!queryOptions) {
		return '';
	}
	const queryParams = [];
	for (const key in queryOptions) {
		if (queryOptions[key] !== null && queryOptions[key] !== undefined) {
			queryParams.push(`${key}=${encodeURIComponent(queryOptions[key])}`);
		}
	}
	if (queryParams.length === 0) {
		return '';
	}
	return `?${queryParams.join('&')}`;
}
