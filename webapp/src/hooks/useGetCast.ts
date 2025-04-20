import { useQuery } from "@tanstack/react-query"

type CastResult = {
	id: number;
	name: string;
	profile_path: string | null;
	total_episode_count: number;
	character_description: string | null;
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

const getCast = async (query: number): Promise<GetCastDto | void> => {
	return fetch(`${import.meta.env.VITE_API_BASE_URL}/cast?query=${query}`)
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

const useGetCast = (query: number) =>
	useQuery({
		queryKey: ["cast", query],
		queryFn: () => getCast(query),
		enabled: !!query,
		retry: false,
	});

export { useGetCast, type CastResult };

