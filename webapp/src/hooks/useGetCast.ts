import { useQuery } from "@tanstack/react-query"

type CastResult = {
	id: number;
	name: string;
	profile_path: string | null;
	total_episode_count: number;
	roles: {
		credit_id: string;
		character: string;
		episode_count: number;
	}[];
}

type GetCastDto = {
	CastDetails: {
		cast: CastResult[];
	}
}

const getCast = async (query: number): Promise<GetCastDto> => {
	return fetch(`${import.meta.env.VITE_API_BASE_URL}/cast?query=${query}`)
		.then((res) => res.json());
};

const useGetCast = (query: number) =>
	useQuery({
		queryKey: ["cast", query],
		queryFn: () => getCast(query),
		enabled: !!query,
	});

export { useGetCast, type CastResult };

