import * as React from "react"
import { Check } from "lucide-react"

import { cn } from "@/lib/utils"
import {
  Command,
  CommandGroup,
  CommandItem,
  CommandList,
} from "@/components/ui/command"
import useGetShow from "@/hooks/useGetShow";

function SeasonAndEpisodeDropdown({ showId }: { showId: number }) {
  const [value, setValue] = React.useState("")
  const { data } = useGetShow(showId);
  const seasons = data?.ShowDetails.seasons || [];

  return (
    <Command>
      <CommandList>
        <CommandGroup>
          {seasons.map((season) => (
            <CommandItem
              key={season.id}
              value={season.id.toString()}
              onSelect={(currentValue) => {
                setValue(currentValue === value ? "" : currentValue)
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
  )
}

export default SeasonAndEpisodeDropdown
