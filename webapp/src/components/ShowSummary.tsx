import useGetSummary from "@/hooks/useGetSummary";

function ShowSummary({ show, season, episode }: { show: string, season: number, episode: number }) {
  const { data, isLoading } = useGetSummary(show, season, episode);

  return (
    <>
      <div className='my-2 text-lg font-semibold'>
        <h1>Summary</h1>
      </div>
      {isLoading ? <p>Loading...</p> : <p className="whitespace-pre-line text-base text-justify">{data?.SummaryResult.response}</p>}
    </>
  )
}

export default ShowSummary;
