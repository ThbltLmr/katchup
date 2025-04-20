import { useQuery } from "@tanstack/react-query"

type SummaryDto = {
	SummaryResult: {
		response: string;
	}
}

const getSummary = async (show: string, season: number, episode: number): Promise<SummaryDto | void> => {
	return fetch(`${import.meta.env.VITE_API_BASE_URL}/summary?query=${show}&season=${season}&episode=${episode}`)
		.then((res) => {
			if (res.status === 200) {
				return res.json()
			}
			else {
				throw new Error();
			}
		})
		.catch((e) => { throw new Error(e) });
};

const useGetSummary = (show: string, season: number, episode: number) =>
	useQuery({
		queryKey: ["summary", show, season, episode],
		queryFn: () => getSummary(show, season, episode),
		enabled: !!show,
		retry: false,
	});

export default useGetSummary;

