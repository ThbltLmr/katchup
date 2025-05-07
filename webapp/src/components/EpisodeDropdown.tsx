import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"

import { SeasonResult } from "@/hooks/useGetShow";

type Props = {
  season: SeasonResult;
  setSelectedEpisode: React.Dispatch<React.SetStateAction<number | undefined>>;
}

function EpisodeDropdown({ season, setSelectedEpisode }: Props) {

  const handleSelectEpisodeValue = (value: number) => {
    setSelectedEpisode(value);
  }


  return (
    <div className={`m-4 transition-all h-full duration-300 ease-in-out ${!!season ? 'w-1/4' : 'w-1/2'}`}>
      <Select onValueChange={(currentValue) => {
        handleSelectEpisodeValue(parseInt(currentValue))
      }}>
        <SelectTrigger className="w-full">
          <SelectValue placeholder="Select a season" />
        </SelectTrigger>
        <SelectContent>
          <SelectGroup>
            {Array.from({ length: season.episode_count }, (_, i) =>
              <SelectItem
                key={i + 1}
                value={(i + 1).toString()}
              >
                Episode {i + 1}
              </SelectItem>
            )}
          </SelectGroup>
        </SelectContent>
      </Select>
    </div >
  )
}

export default EpisodeDropdown
