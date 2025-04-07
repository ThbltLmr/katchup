import {
  Carousel,
  CarouselContent,
  CarouselItem,
  CarouselNext,
  CarouselPrevious,
} from "@/components/ui/carousel"
import { useGetCast } from "@/hooks/useGetCast";

function CastList({ showId }: { showId: number }) {
  const { data } = useGetCast(showId);

  const cast = data?.CastDetails.cast || [];

  return (
    <Carousel>
      <CarouselContent>
        {cast.map((actor) =>
          <CarouselItem key={actor.id} className="basis-1/3">
            {actor.name}
          </CarouselItem>
        )}
      </CarouselContent>
      <CarouselPrevious />
      <CarouselNext />
    </Carousel>
  )
}

export default CastList
