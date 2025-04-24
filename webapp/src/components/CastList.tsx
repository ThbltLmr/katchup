import {
  Carousel,
  CarouselContent,
  CarouselItem,
  CarouselNext,
  CarouselPrevious,
} from "@/components/ui/carousel"
import {
  Card,
  CardContent,
} from "@/components/ui/card"
import { CastResult, useGetCast } from "@/hooks/useGetCast";
import useGetTmdbImage, { ProfileSizes } from "@/hooks/useGetTmdbImage";
import { Skeleton } from "./ui/skeleton";

function CastList({ showId }: { showId: number }) {
  const { data, isLoading, isError } = useGetCast(showId);

  const cast = data?.CastDetails.cast || [];

  const castCards = (cast: CastResult[]) => {
    return (
      cast.map((actor) =>
        <CarouselItem key={actor.id} className="basis-1/5 w-1/6">
          <Card className="center h-[300px]">
            <CardContent className="flex flex-col h-full justify-between">
              {!!actor.profile_path &&
                <img className="h-[120px] w-full rounded-xl object-cover" src={useGetTmdbImage(actor.profile_path, ProfileSizes.w185)} />
              }
              <p className="h-4 text-sm w-full font-bold">{actor.roles[0].character}</p>
              <p className="h-4 text-xs w-full"> ({actor.name})</p>
              <p className="h-20 text-start align-middle text-xs w-full">{(!!actor.character_description && actor.character_description.length > 0) ? actor.character_description : "No description available"}</p>
            </CardContent>
          </Card>
        </CarouselItem>
      ))
  }

  const castSkeleton = () => {
    return (
      <div className="ms-2 flex justify-between w-full">
        {Array.from({ length: 5 }).map((_, i) =>
          <div key={i} className="ms-2 flex flex-col justify-between h-[300px] w-1/5">
            <Skeleton className="h-[150px] w-full rounded-xl" />
            <Skeleton className="h-4 w-full" />
            <Skeleton className="h-4 w-full" />
            <Skeleton className="h-20 w-full" />
          </div>
        )}
      </div>
    )
  }

  const errorContent = () => {
    return (
      <p>Could not retrieve cast data. Please try again later.</p>
    )
  }

  return (
    <>
      <div className='text-start ms-4 my-2 text-2xl font-semibold'>
        <h1>Cast</h1>
      </div>
      {isError ? errorContent() :
        <Carousel>
          <CarouselContent className="p-4">
            {isLoading ? castSkeleton() : castCards(cast)}
          </CarouselContent>
          <CarouselPrevious />
          <CarouselNext />
        </Carousel>
      }
    </>
  )
}

export default CastList
