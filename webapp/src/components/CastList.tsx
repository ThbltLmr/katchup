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
  CardFooter,
} from "@/components/ui/card"
import { CastResult, useGetCast } from "@/hooks/useGetCast";
import useGetTmdbImage, { ProfileSizes } from "@/hooks/useGetTmdbImage";
import { Skeleton } from "./ui/skeleton";

function CastList({ showId }: { showId: number }) {
  const { data, isLoading } = useGetCast(showId);

  const cast = data?.CastDetails.cast || [];

  const castCards = (cast: CastResult[]) => {
    return (
      cast.map((actor) =>
        <CarouselItem key={actor.id} className="basis-1/5 w-1/6">
          <Card className="h-[300px]">
            <CardContent className="center">
              {!!actor.profile_path &&
                <img className="h-[120px] w-full rounded-xl object-cover" src={useGetTmdbImage(actor.profile_path, ProfileSizes.w185)} />
              }
              <div className="py-2 text-start">
                <span className="h-4 text-sm w-full font-bold">{actor.roles[0].character}</span>
                <span className="h-4 text-xs w-full"> ({actor.name})</span>
                <p className="h-4 pt-4 text-start text-xs w-full">{(!!actor.character_description && actor.character_description.length > 0) ? actor.character_description : "No description available"}</p>
              </div>
            </CardContent>
            <CardFooter>
            </CardFooter>
          </Card>

        </CarouselItem>
      ))
  }

  const castSkeleton = () => {
    return (
      <div className="flex justify-between w-full">
        {Array.from({ length: 5 }).map(() =>
          <div className="flex flex-col h-[300px] basis-1/5 p-2 w-1/6">
            <Skeleton className="h-[150px] w-full rounded-xl" />
            <div className="my-2">
              <Skeleton className="h-4 w-full" />
              <Skeleton className="h-4 w-full" />
            </div>
          </div>
        )}
      </div>
    )
  }

  return (
    <Carousel>
      <CarouselContent className="p-2">
        {isLoading ? castSkeleton() : castCards(cast)}
      </CarouselContent>
      <CarouselPrevious />
      <CarouselNext />
    </Carousel>
  )
}

export default CastList
