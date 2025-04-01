import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { useGetShow, SeasonResult } from "@/hooks/useGetShow";

function SeasonDropdown({ showId, setSelectedSeason, season }: { showId: number, season: SeasonResult | undefined, setSelectedSeason: React.Dispatch<React.SetStateAction<SeasonResult | undefined>> }) {
  const { data } = useGetShow(showId);

  const seasons = data?.ShowDetails.seasons || [];

  const handleSelectSeasonValue = (value: number) => {
    setSelectedSeason(seasons.find((season) => season.id === value));
  }


  return (
    <div className={`m-4 transition-all h-full duration-300 ease-in-out ${!!season ? 'w-1/4' : 'w-1/2'}`}>
      <Select onValueChange={(currentValue) => {
        handleSelectSeasonValue(parseInt(currentValue))
      }}>
        <SelectTrigger className="w-full">
          <SelectValue placeholder="Select a season" />
        </SelectTrigger>
        <SelectContent>
          <SelectGroup>
            {seasons.map((season, i) => (
              <SelectItem
                value={season.id.toString()}
              >
                Season {i + 1}
              </SelectItem>
            ))}
          </SelectGroup>
        </SelectContent>
      </Select>
    </div>
  )
}

export default SeasonDropdown
