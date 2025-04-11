import useGetSummary from "@/hooks/useGetSummary";

function ShowSummary({ show, season, episode }: { show: string, season: number, episode: number }) {
  const { data, isLoading } = useGetSummary(show, season, episode);

  return (
    <>
      {isLoading ? <p>Loading...</p> : <p>{data?.SummaryResult.response}</p>}
    </>
  )
}

export default ShowSummary;
