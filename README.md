## Description
🎬 Media List

Manage and explore your favorite movies and TV shows with ease. This web application combines a Rust backend with SvelteKit on the frontend to provide a seamless and responsive user experience.

🌐 Features:
- Browse and filter a collection of media items.
- Add, edit, and delete your media entries.
- Generate random movie and show suggestions.

🚀 Rust & SvelteKit

## Installation and Setup

### Prerequisites

Before you begin, ensure you have met the following requirements:

- [Rust](https://www.rust-lang.org/tools/install) installed on your system.
- [Node.js](https://nodejs.org/) and [npm](https://www.npmjs.com/) installed on your system.
- [MongoDB](https://www.mongodb.com/try/download/community) database set up.

```shell
git clone https://github.com/vaishnav-mk/movie-api.git
cd movie-api
```

### Backend (Rust)

Navigate to the Backend Directory:

```shell
cd backend
```

Install Rust Dependencies:

```shell
cargo build
```

Run the Backend Server:

```shell
cargo run
```

The backend server should now be running at http://localhost:8080.

Frontend (SvelteKit)
Navigate Back to the Project Root Directory:

```shell
cd ..
```

### Frontend (SvelteKit)

Navigate to the Frontend Directory:

```shell
cd frontend
```

Install Frontend Dependencies:

```shell
npm install
```

Start the SvelteKit Development Server:

```shell
npm run dev
```

🌟 The frontend development server should now be running at http://localhost:5173.

### Running the Application
Open your web browser and visit http://localhost:5173 to access the Media List application.

You can now browse, add, edit, delete, and explore your favorite movies and TV shows seamlessly.

### Additional Configuration
Customize the backend and frontend configurations in the respective directories as needed.

