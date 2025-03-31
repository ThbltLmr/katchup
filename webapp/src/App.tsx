import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import './App.css'
import { ModeToggle } from './components/providers/ModeToggle'
import { ThemeProvider } from './components/providers/ThemeProvider'
import SearchBar from './components/SearchBar'
import { useState } from 'react'

function App() {
  const queryClient = new QueryClient()
  const [selectedShowId, setSelectedShowId] = useState(0);

  return (
    <>
      <QueryClientProvider client={queryClient}>
        <ThemeProvider defaultTheme='system' storageKey='vite-ui-theme'>
          <ModeToggle />
          <SearchBar setSelectedShowId={setSelectedShowId} />
        </ThemeProvider>
      </QueryClientProvider>
    </>
  )
}

export default App
