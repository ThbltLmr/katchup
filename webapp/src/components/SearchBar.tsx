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

type ShowResult = {
  name: string;
  id: number;
}

function SearchBar() {
  const [open, setOpen] = useState(false);
  const [search, setSearch] = useState<string>('');

  const { data, isPending, refetch } = useSearchShows(search);

  const handleSearch = (value: string) => {
    if (value === '') {
      setOpen(false);
      return;
    }

    setSearch(value);
    setOpen(true);
    refetch();
  }

  return (
    <Command>
      <CommandInput placeholder="Search for a show..." onValueChange={handleSearch} />
      <CommandList hidden={!open}>
        <CommandEmpty>No results found.</CommandEmpty>
        {!!data?.SearchResults.results.length && !isPending &&
          <CommandGroup heading="Search results">
            {data.SearchResults.results.map((show) => <CommandItem key={show.id}>{show.name}</CommandItem>)}
          </CommandGroup>
        }
      </CommandList>
    </Command>

  )
}

export default SearchBar
