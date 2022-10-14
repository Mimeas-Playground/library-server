<<<<<<< Updated upstream
import { useState } from 'react'
=======
import { MouseEvent, useEffect, useState } from 'react'
>>>>>>> Stashed changes
import reactLogo from './assets/react.svg'
import './App.css'

function App() {
<<<<<<< Updated upstream
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

=======
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

function BookForm(props) {
  const [title, setTitle] = useState('');

  function submit(e: MouseEvent<HTMLButtonElement, MouseEvent>) {
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

>>>>>>> Stashed changes
export default App
