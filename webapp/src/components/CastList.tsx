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
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import { useGetCast } from "@/hooks/useGetCast";
import useGetTmdbImage, { ProfileSizes } from "@/hooks/useGetTmdbImage";

function CastList({ showId }: { showId: number }) {
  const { data } = useGetCast(showId);

  const cast = data?.CastDetails.cast || [];

  return (
    <Carousel>
      <CarouselContent className="p-4">
        {cast.map((actor) =>
          <CarouselItem key={actor.id} className="basis-1/4 w-1/5">
            <Card>
              <CardHeader>
                <CardTitle className="text-base">{actor.roles[0].character}</CardTitle>
              </CardHeader>
              <CardContent className="center">
                {!!actor.character_description &&
                  <p>{actor.character_description}</p>
                }
                {!!actor.profile_path &&
                  <img className="mx-auto object-scale-down" src={useGetTmdbImage(actor.profile_path, ProfileSizes.w185)} />
                }
              </CardContent>
              <CardFooter>
                <p className="text-center w-full text-sm">Played by {actor.name}</p>
              </CardFooter>
            </Card>

          </CarouselItem>
        )}
      </CarouselContent>
      <CarouselPrevious />
      <CarouselNext />
    </Carousel>
  )
}

export default CastList
