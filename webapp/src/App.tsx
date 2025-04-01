import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import './App.css'
import { ModeToggle } from './components/providers/ModeToggle'
import { ThemeProvider } from './components/providers/ThemeProvider'
import SearchBar from './components/SearchBar'
import { useState } from 'react'
import SeasonAndEpisodeDropdown from './components/SeasonAndEpisodeDropDown'

function App() {
  const queryClient = new QueryClient()
  const [selectedShowId, setSelectedShowId] = useState(0);

  return (
    <>
      <QueryClientProvider client={queryClient}>
        <ThemeProvider defaultTheme='system' storageKey='vite-ui-theme'>
          <div className='mx-auto relative w-full'>
            <div className='absolute top-0 right-0'>
              <ModeToggle />
            </div>
            <div className='my-8 text-2xl font-semibold'>
              <h1>Katchup</h1>
            </div>
            <div className='flex transition-all duration-300 ease-in-out'>
              <SearchBar selectedShowId={selectedShowId} setSelectedShowId={setSelectedShowId} />
              {selectedShowId > 0 &&
                <SeasonAndEpisodeDropdown showId={selectedShowId} />
              }
            </div>
          </div>
        </ThemeProvider>
      </QueryClientProvider>
    </>
  )
}

export default App
