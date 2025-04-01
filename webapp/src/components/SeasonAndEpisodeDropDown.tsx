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
  const [seasonSelectOpen, setSeasonSelectOpen] = useState(true);
  const [episodeSelectOpen, setEpisodeSelectOpen] = useState(true);
  const [selectedSeason, setSelectedSeason] = useState<SeasonResult | undefined>(undefined);
  const [selectedEpisode, setSelectedEpisode] = useState<number | undefined>(undefined);

  const { data } = useGetShow(showId);

  const seasons = data?.ShowDetails.seasons || [];

  const handleSelectSeasonValue = (value: number) => {
    setSeasonValue(value.toString());
    setSelectedSeason(seasons.find((season) => season.id === value));
    setSeasonSelectOpen(false);
  }

  const handleSelectEpisodeValue = (value: number) => {
    setEpisodeValue(value.toString());
    setSelectedEpisode(value);
    setEpisodeSelectOpen(false);
  }


  return (
    <>
      <Command className={`m-4 transition-all duration-300 ease-in-out ${showId > 0 ? !!selectedSeason ? 'w-1/4' : 'w-1/2' : 'w-full'}`}>
        <CommandList>
          <CommandInput placeholder="Choose a season" value={!!selectedSeason ? `Season ${(seasons.findIndex((season) => season.id === selectedSeason.id)! + 1).toString()}` : undefined} />
          <CommandGroup hidden={!seasonSelectOpen}>
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
        <Command className={`m-4 transition-all duration-300 ease-in-out ${!!selectedSeason ? 'w-1/4' : 'w-1/2'}`}>
          <CommandList>
            <CommandInput placeholder="Choose an episode" value={!!selectedEpisode ? `Episode ${selectedEpisode}` : undefined} />
            <CommandGroup hidden={!episodeSelectOpen}>
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
