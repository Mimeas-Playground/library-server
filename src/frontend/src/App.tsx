import { MouseEvent, useEffect, useState } from 'react'
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

  function getBooks() {
    console.log("I run!")

    fetch("/api")
      .then(async (res) => {
        setBooks(await res.json())
      })
  }
  useEffect(getBooks, [])

  return (<>
    <h3>Add a book</h3>
    <BookForm update={()=>getBooks()}/>
    <hr/>
    <ul className="book">
      {
        books.map(
          (b: Book) => {
            return <li className='book'>{b.title}</li>
          }
        )
      }
    </ul>
    </>)
}

function BookForm(props: { update: (() => void) | null | undefined; }) {
  const [title, setTitle] = useState('');

  function submit(e: MouseEvent<HTMLButtonElement, globalThis.MouseEvent>) {
    e.preventDefault();
    fetch("/api", {
      method: "POST",
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({title})
    }).finally(props.update)
  }

  return (
    <form className="addBook">
      <input type="text" placeholder="Title" onChange={(e) => setTitle(e.target.value)}/>
      <button onClick={(e)=>submit(e)}>Add</button>
    </form>
  )
}



interface Book {
  title: string
}

export default App
