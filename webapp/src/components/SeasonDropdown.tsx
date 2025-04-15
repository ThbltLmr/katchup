import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { useGetShow, SeasonResult } from "@/hooks/useGetShow";
import React from "react";

function SeasonDropdown({ showId, setSelectedSeason, season, setSelectedSeasonNumber }: { showId: number, season: SeasonResult | undefined, setSelectedSeason: React.Dispatch<React.SetStateAction<SeasonResult | undefined>>, setSelectedSeasonNumber: React.Dispatch<number | undefined> }) {
  const { data } = useGetShow(showId);

  const seasons = data?.ShowDetails.seasons || [];

  const handleSelectSeasonValue = (value: string) => {
    const seasonIndex = parseInt(value);
    const season = seasons[seasonIndex]
    setSelectedSeason(season);
    setSelectedSeasonNumber(seasonIndex + 1);
  }


  return (
    <div className={`m-4 transition-all h-full duration-300 ease-in-out ${!!season ? 'w-1/4' : 'w-1/2'}`}>
      <Select onValueChange={handleSelectSeasonValue}>
        <SelectTrigger className="w-full">
          <SelectValue placeholder="Select a season" />
        </SelectTrigger>
        <SelectContent>
          <SelectGroup>
            {seasons.map((season, i) => (
              <SelectItem
                key={season.id}
                value={i.toString()}
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
