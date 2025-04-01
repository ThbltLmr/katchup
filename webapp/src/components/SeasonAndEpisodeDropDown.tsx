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
  const [seasonValue, setSeasonValue] = useState("")
  const [episodeValue, setEpisodeValue] = useState("")
  const [selectedSeason, setSelectedSeason] = useState<SeasonResult | undefined>(undefined);
  const [selectedEpisode, setSelectedEpisode] = useState<number | undefined>(undefined);

  const { data } = useGetShow(showId);

  const seasons = data?.ShowDetails.seasons || [];

  const handleSelectSeasonValue = (value: number) => {
    setSeasonValue(value.toString());
    setSelectedSeason(seasons.find((season) => season.id === value));
  }

  const handleSelectEpisodeValue = (value: number) => {
    setEpisodeValue(value.toString());
    setSelectedEpisode(value);
  }


  return (
    <>
      <Command>
        <CommandList>
          <CommandInput placeholder="Choose a season" />
          <CommandGroup>
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

      {!!selectedSeason &&
        <Command>
          <CommandList>
            <CommandInput placeholder="Choose an episode" />
            <CommandGroup>
              {Array.from({ length: selectedSeason.episode_count }, (_, i) => (
                <CommandItem
                  key={i}
                  value={(i + 1).toString()}
                  onSelect={(currentValue) => {
                    handleSelectEpisodeValue(currentValue === episodeValue ? 0 : i + 1)
                  }}
                >
                  <Check
                    className={cn(
                      "mr-2 h-4 w-4",
                      episodeValue === i.toString() ? "opacity-100" : "opacity-0"
                    )}
                  />
                  Episode {i + 1}
                </CommandItem>
              ))}
            </CommandGroup>
          </CommandList>
        </Command>
      }
    </>
  )
}

export default SeasonAndEpisodeDropdown
