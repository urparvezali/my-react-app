import { useEffect, useState } from "react";
import "./App.css";

function App() {
  const [todos, setTodos] = useState([]);
  const [title, setTitle] = useState("");

  useEffect(() => {
    fetch("http://localhost:8000/api/todos")
      .then((response) => {
        if (!response.ok) {
          throw new Error("Network response was not ok");
        }
		console.log('Response: ',response);
        return response.json();
      })
      .then((data) => {
        console.log("Data :", data);
        setTodos(data);
      })
      .catch((error) => {
        console.error("Error fetching todos:", error);
      });
  }, []);

  const addTodo = () => {
    fetch("http://localhost:8000/api/todos", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ title }),
    })
      .then((response) => {
        if (!response.ok) {
          throw new Error("Network response was not ok");
        }
        return response.json();
      })
      .then((data) => {
        console.log("Added todo:", data);
        setTodos([...todos, data]);
        setTitle("");
      })
      .catch((error) => {
        console.error("Error adding todo:", error);
      });
  };

  return (
    <div className="App">
      <h1>Todo List</h1>
      <div>
        <input
          type="text"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          placeholder="Enter todo"
        />
        <button onClick={addTodo}>Add Todo</button>
      </div>
      <ul>
        {todos.map((todo) => (
          <li key={todo.id}>{todo.title}</li>
        ))}
      </ul>
    </div>
  );
}

export default App;
