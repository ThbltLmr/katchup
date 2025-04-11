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
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import { CastResult, useGetCast } from "@/hooks/useGetCast";
import useGetTmdbImage, { ProfileSizes } from "@/hooks/useGetTmdbImage";
import { Skeleton } from "./ui/skeleton";

function CastList({ showId }: { showId: number }) {
  const { data, isError, isLoading } = useGetCast(showId);

  const cast = data?.CastDetails.cast || [];

  const castCards = (cast: CastResult[]) => {
    return (
      cast.map((actor) =>
        <CarouselItem key={actor.id} className="basis-1/3 w-1/4">
          <Card className="h-[400px]">
            <CardContent className="center">
              {!!actor.profile_path &&
                <img className="h-[250px] w-full rounded-xl object-cover" src={useGetTmdbImage(actor.profile_path, ProfileSizes.original)} />
              }
              <div className="py-4 text-start">
                <span className="h-4 text-xl w-full font-bold">{actor.roles[0].character}</span>
                <span className="h-4 text-base w-full"> ({actor.name})</span>
                <p className="h-4 pt-4 text-start text-sm w-full">{(!!actor.character_description && actor.character_description.length > 0) ? actor.character_description : "No description available"}</p>
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
      <div className="flex flex-col space-y-3">
        <Skeleton className="h-[250px] w-[250px] rounded-xl" />
        <div className="space-y-2">
          <Skeleton className="h-4 w-[250px]" />
          <Skeleton className="h-4 w-[200px]" />
        </div>
      </div>
    )
  }

  return (
    <Carousel>
      <CarouselContent className="p-4">
        {isLoading ? castSkeleton() : castCards(cast)}
      </CarouselContent>
      <CarouselPrevious />
      <CarouselNext />
    </Carousel>
  )
}

export default CastList
