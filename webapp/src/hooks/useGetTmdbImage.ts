enum PosterSizes {
	w92 = "w92",
	w154 = "w154",
	w185 = "w185",
	w342 = "w342",
	w500 = "w500",
	w780 = "w780",
	original = "original"
}

enum ProfileSizes {
	w45 = "w45",
	w185 = "w185",
	h632 = "h632",
	original = "original"
}

const useGetTmdbImage = (filePath: string, fileSize: ProfileSizes | PosterSizes): string =>
	`${import.meta.env.VITE_TMDB_IMAGE_BASE_URL}/${fileSize}/${filePath}`
	;

export default useGetTmdbImage;
export { PosterSizes, ProfileSizes }

