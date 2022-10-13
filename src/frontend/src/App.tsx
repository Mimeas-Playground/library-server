import { useEffect, useState } from 'react'
import reactLogo from './assets/react.svg'
import './App.css'

function App() {
  return (<>
    <h1>SHHH!!!</h1>
    <h2>This is a library</h2>
    <hr/>
    <BookList/>
    </>)
}

function BookList() {
  const [books, setBooks] = useState([]);
  useEffect(() => {
    fetch("/api")
      .then(async (res) => {
        setBooks(await res.json())
      })
  }, [])

  return (
    <ul className="book">
      {
        books.map(
          (b: Book) => {
            return <h3>{b.title}</h3>
          }
        )
      }
    </ul>
  )
}

interface Book {
  title: string
}

export default App
