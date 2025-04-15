import { useState } from "react"
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@/components/ui/command"
import { ShowResult, useSearchShows } from "@/hooks/useSearchShows";

function SearchBar({ selectedShowId, setSelectedShowId, setSelectedShowName }: { selectedShowId: number, setSelectedShowId: React.Dispatch<React.SetStateAction<number>>, setSelectedShowName: React.Dispatch<React.SetStateAction<string>> }) {
  const [open, setOpen] = useState(false);
  const [search, setSearch] = useState<string>('');

  const { data, isPending, refetch } = useSearchShows(search);
  const shows = data?.SearchResults.results;

  const handleSearch = (value: string) => {
    setSelectedShowId(0);

    if (value === '') {
      setOpen(false);
      return;
    }

    setSearch(value);
    setOpen(true);
    refetch();
  }

  const handleShowSelect = (show: ShowResult) => {
    setSelectedShowId(show.id);
    setSelectedShowName(search);
    setOpen(false);
  }

  return (
    <Command className={`m-4 h-full transition-all duration-300 ease-in-out ${selectedShowId > 0 ? 'w-1/2' : 'w-full'}`}>
      <CommandInput placeholder="Search for a show..." onValueChange={handleSearch} value={shows?.find((show) => show.id === selectedShowId)?.name || undefined} />
      <CommandList hidden={!open}>
        <CommandEmpty>No results found.</CommandEmpty>
        {!!shows && !isPending &&
          <CommandGroup heading="Search results">
            {shows.map((show) => <CommandItem key={show.id} onSelect={() => handleShowSelect(show)}>{show.name}</CommandItem>)}
          </CommandGroup>
        }
      </CommandList>
    </Command>
  )
}

export default SearchBar
