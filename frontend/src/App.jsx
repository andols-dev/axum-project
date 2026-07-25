import { useEffect, useState } from "react";
import "./App.css";
import Layout from "./components/layout/Layout.jsx";
import { Link } from "react-router";

function App() {
  const [message, setMessage] = useState("");

  useEffect(() => {
    fetch("http://localhost:3000/api/home")
      .then((response) => response.json())
      .then((data) => setMessage(data.message))
      .catch((error) => console.error("Error fetching message:", error));
  }, []);
  return (
    <>
      <Layout>
        <h1 className="text-3xl font-bold ">Axum Frontend project</h1>
        <p>{message}</p>
        <Link
          to="/login"
          className="cursor-pointer text-blue-500 hover:text-blue-700"
        >
          Login
        </Link>
      </Layout>
    </>
  );
}

export default App;
