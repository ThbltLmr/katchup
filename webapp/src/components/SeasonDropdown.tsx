
import { Check } from "lucide-react"

import { cn } from "@/lib/utils"
import {
  Command,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@/components/ui/command"
import { useGetShow, SeasonResult } from "@/hooks/useGetShow";
import { useState } from "react";

function SeasonAndEpisodeDropdown({ showId, setSelectedSeason, season }: { showId: number, season: SeasonResult | undefined, setSelectedSeason: React.Dispatch<React.SetStateAction<SeasonResult | undefined>> }) {
  const [seasonValue, setSeasonValue] = useState("")
  const [open, setOpen] = useState(true);

  const { data } = useGetShow(showId);

  const seasons = data?.ShowDetails.seasons || [];

  const handleSelectSeasonValue = (value: number) => {
    setSeasonValue(value.toString());
    setSelectedSeason(seasons.find((season) => season.id === value));
    setOpen(false);
  }


  return (
    <Command className={`m-4 transition-all h-full duration-300 ease-in-out ${!!season ? 'w-1/4' : 'w-1/2'}`}>
      <CommandList>
        <CommandInput placeholder="Choose a season" onClick={() => setOpen(true)} value={!!season ? `Season ${(seasons.findIndex((season) => season.id === season.id)! + 1).toString()}` : undefined} />
        <CommandGroup hidden={!open}>
          {seasons.map((season, i) => (
            <CommandItem
              key={season.id}
              value={season.id.toString()}
              onSelect={(currentValue) => {
                handleSelectSeasonValue(currentValue === seasonValue ? 0 : season.id)
              }}
            >
              <Check
                className={cn(
                  "mr-2 h-4 w-4",
                  seasonValue === season.id.toString() ? "opacity-100" : "opacity-0"
                )}
              />
              Season {i + 1}
            </CommandItem>
          ))}
        </CommandGroup>
      </CommandList>
    </Command>
  )
}

export default SeasonAndEpisodeDropdown
