import { SeasonResult } from "@/hooks/useGetShow";
import { Dispatch, SetStateAction } from "react";
import { Button } from "./ui/button";
import { X } from 'lucide-react';

type Props = {
  setSelectedShowId: Dispatch<SetStateAction<number>>;
  setSelectedShowName: Dispatch<SetStateAction<string>>;
  setSelectedSeason: Dispatch<SetStateAction<SeasonResult | undefined>>;
  setSelectedSeasonNumber: Dispatch<SetStateAction<number | undefined>>;
  setSelectedEpisode: Dispatch<SetStateAction<number | undefined>>;
}

function ClearButton({ setSelectedSeason, setSelectedSeasonNumber, setSelectedShowId, setSelectedEpisode, setSelectedShowName }: Props) {
  const handleReset = () => {
    setSelectedShowId(0)
    setSelectedShowName('')
    setSelectedSeason(undefined)
    setSelectedSeasonNumber(undefined)
    setSelectedEpisode(undefined)
  }

  return (
    <>
      <Button className='mx-4 rounded-full bg-transparent text-primary border-primary border-1 hover:bg-transparent hover:font-semibold hover:px-5' onClick={handleReset}>
        <X />
        Clear selection
      </Button>
    </>
  )
}

export default ClearButton;
