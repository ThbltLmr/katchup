import { Check } from "lucide-react"

import { cn } from "@/lib/utils"
import {
  Command,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@/components/ui/command"
import { SeasonResult } from "@/hooks/useGetShow";
import { useState } from "react";

function SeasonAndEpisodeDropdown({ season, episode, setSelectedEpisode }: { season: SeasonResult, episode: number | undefined, setSelectedEpisode: React.Dispatch<React.SetStateAction<number | undefined>> }) {
  const [episodeValue, setEpisodeValue] = useState("")

  const handleSelectEpisodeValue = (value: number) => {
    setEpisodeValue(value.toString());
    setSelectedEpisode(value);
  }


  return (
    <Command className={'m-4 h-full transition-all duration-300 ease-in-out w-1/4'}>
      <CommandList>
        <CommandInput autoFocus placeholder="Choose an episode" value={!!episode ? `Episode ${episode}` : undefined} />
        <CommandGroup>
          {Array.from({ length: season.episode_count }, (_, i) => (
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
  )
}

export default SeasonAndEpisodeDropdown
