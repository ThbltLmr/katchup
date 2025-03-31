import { useQuery } from "@tanstack/react-query"

type SeasonResult = {
	id: number;
	name: string;
	episode_count: number;
}

type GetShowDto = {
	ShowDetails: {
		number_of_seasons: number;
		number_of_episodes: number;
		seasons: SeasonResult[];
	}
}

const getShow = async (query: string): Promise<GetShowDto> => {
	return fetch(`${import.meta.env.VITE_API_BASE_URL}/shows?query=${query}`)
		.then((res) => res.json());
};

const useGetShow = (query: string) =>
	useQuery({
		queryKey: ["shows", query],
		queryFn: () => getShow(query),
		enabled: !!query,
	});

export default useGetShow;

