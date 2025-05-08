import { useQuery } from "@tanstack/react-query"

type SummaryDto = {
	SummaryResult: string;
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

const useGetSummary = (showId: number, season: number, episode: number) =>
	useQuery({
		queryKey: ["summary", showId, season, episode],
		queryFn: () => getSummary(showId.toString(), season, episode),
		enabled: !!showId,
		retry: false,
	});

export default useGetSummary;

