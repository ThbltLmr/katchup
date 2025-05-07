import useGetSummary from "@/hooks/useGetSummary";
import { Skeleton } from "./ui/skeleton";

type Props = {
  show: string;
  season: number;
  episode: number;
}

function ShowSummary({ show, season, episode }: Props) {
  const { data, isLoading } = useGetSummary(show, season, episode);

  const summarySkeleton = () =>
    <div>
      <Skeleton className="h-8 mx-4 my-2" />
      <Skeleton className="h-8 mx-4 my-2" />
      <Skeleton className="h-8 mx-4 my-2" />
    </div>

  return (
    <>
      <div className='mx-4 text-start my-2 text-2xl font-semibold'>
        <h1>Summary</h1>
      </div>
      {isLoading ? summarySkeleton() : <p className="p-4 whitespace-pre-line text-base text-justify">{data?.SummaryResult}</p>}
    </>
  )
}

export default ShowSummary;
