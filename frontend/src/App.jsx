import "./App.css";
import { Link } from "react-router";

function App() {
  return (
    <>
      <h1 class="text-3xl font-bold ">Axum Frontend project</h1>
      <Link
        to="/login"
        class="cursor-pointer text-blue-500 hover:text-blue-700"
      >
        Login
      </Link>
    </>
  );
}

export default App;
