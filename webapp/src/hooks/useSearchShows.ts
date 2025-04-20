import { useQuery } from "@tanstack/react-query"

type ShowResult = {
	name: string;
	id: number;
}

type SearchShowsDto = {
	SearchResults: {
		results: ShowResult[];
	}
}

const searchShows = async (query: string): Promise<SearchShowsDto> => {
	return fetch(`${import.meta.env.VITE_API_BASE_URL}/search?query=${query}`)
		.then((res) => res.json()).catch((e) => { throw new Error(e) });
};

const useSearchShows = (query: string) =>
	useQuery({
		queryKey: ["search", query],
		queryFn: () => searchShows(query),
		enabled: !!query,
	});

export { useSearchShows, type ShowResult };

