import './App.css'
import { ModeToggle } from './components/providers/ModeToggle'
import { ThemeProvider } from './components/providers/ThemeProvider'
import SearchBar from './components/SearchBar'

function App() {
  return (
    <>
      <ThemeProvider defaultTheme='system' storageKey='vite-ui-theme'>
        <ModeToggle />
        <SearchBar />
      </ ThemeProvider>
    </>
  )
}

export default App
