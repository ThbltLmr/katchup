import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import './App.css'
import { ModeToggle } from './components/providers/ModeToggle'
import { ThemeProvider } from './components/providers/ThemeProvider'
import SearchBar from './components/SearchBar'
import { useState } from 'react'
import SeasonDropdown from './components/SeasonDropdown'
import EpisodeDropdown from './components/EpisodeDropdown'
import { SeasonResult } from './hooks/useGetShow'
import CastList from './components/CastList'
import ShowSummary from './components/ShowSummary'
import ClearButton from './components/ClearButton'

function App() {
  const queryClient = new QueryClient()
  const [selectedShowId, setSelectedShowId] = useState(0);
  const [selectedSeason, setSelectedSeason] = useState<SeasonResult | undefined>(undefined);
  const [selectedSeasonNumber, setSelectedSeasonNumber] = useState<number | undefined>(undefined);
  const [selectedEpisode, setSelectedEpisode] = useState<number | undefined>(undefined);

  return (
    <>
      <QueryClientProvider client={queryClient}>
        <ThemeProvider defaultTheme='system' storageKey='vite-ui-theme'>
          <div className='mx-auto relative w-full'>
            <div className='absolute top-0 right-4'>
              <ModeToggle />
            </div>
            <div className='mt-8 mb-16 text-6xl font-semibold'>
              <h1>Katchup</h1>
              <h1 className='text-2xl font-normal italic mt-8'>Catch up to your favorite shows with AI-generated, spoiler-free summaries</h1>
            </div>
            <div className='text-start flex flex-row ms-4 my-2 text-2xl font-semibold'>
              <h1>What do you want to catch up on?</h1>
              {(!!selectedEpisode || !!selectedShowId || !!selectedSeasonNumber) &&
                <ClearButton setSelectedEpisode={setSelectedEpisode} setSelectedShowId={setSelectedShowId} setSelectedSeason={setSelectedSeason} setSelectedSeasonNumber={setSelectedSeasonNumber} />
              }
            </div>
            <div className='flex transition-all duration-300 ease-in-out'>
              <SearchBar selectedShowId={selectedShowId} setSelectedShowId={setSelectedShowId} setSelectedSeason={setSelectedSeason} setSelectedEpisode={setSelectedEpisode} />
              {selectedShowId > 0 &&
                <SeasonDropdown showId={selectedShowId} setSelectedSeason={setSelectedSeason} season={selectedSeason} setSelectedSeasonNumber={setSelectedSeasonNumber} />
              }
              {!!selectedSeason &&
                <EpisodeDropdown season={selectedSeason} setSelectedEpisode={setSelectedEpisode} />
              }
            </div>
            {!!selectedEpisode && !!selectedShowId && !!selectedSeasonNumber &&
              <div>
                <CastList showId={selectedShowId} />
                <ShowSummary showId={selectedShowId} season={selectedSeasonNumber} episode={selectedEpisode} />
              </div>
            }
          </div>
        </ThemeProvider>
      </QueryClientProvider >
    </>
  )
}

export default App
