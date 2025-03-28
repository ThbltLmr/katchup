import { useState } from "react"
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@/components/ui/command"

type ShowResult = {
  name: string;
  id: number;
}

function SearchBar() {
  const [open, setOpen] = useState(false);
  const [shows, setShows] = useState<ShowResult[]>([]);

  const handleSearch = (value: string) => {
    if (value === '') {
      setOpen(false);
      setShows([]);
      return;
    }

    setShows([
      {
        id: 1,
        name: 'Game of thrones'
      },
      {
        id: 2,
        name: 'Peaky'
      },
    ]);

    setOpen(true);
  }

  return (
    <Command>
      <CommandInput placeholder="Search for a show..." onValueChange={handleSearch} />
      <CommandList hidden={!open}>
        <CommandEmpty>No results found.</CommandEmpty>
        {!!shows.length &&
          <CommandGroup heading="Search results">
            {shows.map((show) => <CommandItem key={show.id}>{show.name}</CommandItem>)}
          </CommandGroup>
        }
      </CommandList>
    </Command>

  )
}

export default SearchBar
