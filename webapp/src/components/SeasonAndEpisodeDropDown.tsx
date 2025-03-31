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

function SeasonAndEpisodeDropdown({ showId }: { showId: number }) {
  const [value, setValue] = useState("")
  const [selectedSeason, setSelectedSeason] = useState<SeasonResult | undefined>(undefined);
  const { data } = useGetShow(showId);
  const seasons = data?.ShowDetails.seasons || [];

  const handleSelectValue = (value: number) => {
    setValue(value.toString());
    setSelectedSeason(seasons.find((season) => season.id === value));
  }


  return (
    <>
      <Command>
        <CommandList>
          <CommandInput placeholder="Choose a season" />
          <CommandGroup>
            {seasons.map((season) => (
              <CommandItem
                key={season.id}
                value={season.id.toString()}
                onSelect={(currentValue) => {
                  handleSelectValue(currentValue === value ? 0 : season.id)
                }}
              >
                <Check
                  className={cn(
                    "mr-2 h-4 w-4",
                    value === season.id.toString() ? "opacity-100" : "opacity-0"
                  )}
                />
                {season.name}
              </CommandItem>
            ))}
          </CommandGroup>
        </CommandList>
      </Command>

      {!!selectedSeason &&
        <Command>
          <CommandList>
            <CommandInput placeholder="Choose an episode" />
            <CommandGroup>
              {//todo - loop over selectedSeason.number_of_episodes and create one command item for each}
            </CommandGroup>
          </CommandList>
        </Command>
      }
    </>
  )
}

export default SeasonAndEpisodeDropdown
