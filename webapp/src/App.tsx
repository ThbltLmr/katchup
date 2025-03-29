import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import './App.css'
import { ModeToggle } from './components/providers/ModeToggle'
import { ThemeProvider } from './components/providers/ThemeProvider'
import SearchBar from './components/SearchBar'

function App() {
  const queryClient = new QueryClient()
  return (
    <>
      <QueryClientProvider client={queryClient}>
        <ThemeProvider defaultTheme='system' storageKey='vite-ui-theme'>
          <ModeToggle />
          <SearchBar />
        </ThemeProvider>
      </QueryClientProvider>
    </>
  )
}

export default App
