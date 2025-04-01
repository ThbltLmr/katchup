import { useState } from "react"
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@/components/ui/command"
import useSearchShows from "@/hooks/useSearchShows";

function SearchBar({ selectedShowId, setSelectedShowId }: { selectedShowId: number, setSelectedShowId: React.Dispatch<React.SetStateAction<number>> }) {
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

  const handleShowSelect = (value: number) => {
    setSelectedShowId(value);
    setOpen(false);
  }

  return (
    <Command>
      <CommandInput placeholder="Search for a show..." onValueChange={handleSearch} value={shows?.find((show) => show.id === selectedShowId)?.name || undefined} />
      <CommandList hidden={!open}>
        <CommandEmpty>No results found.</CommandEmpty>
        {!!shows && !isPending &&
          <CommandGroup heading="Search results">
            {data.SearchResults.results.map((show) => <CommandItem key={show.id} onSelect={() => handleShowSelect(show.id)}>{show.name}</CommandItem>)}
          </CommandGroup>
        }
      </CommandList>
    </Command>
  )
}

export default SearchBar
