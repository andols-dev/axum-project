import "./App.css";
import Layout from "./components/layout/Layout.jsx";
import { Link } from "react-router";

function App() {
  return (
    <>
      <Layout>
        <h1 className="text-3xl font-bold ">Axum Frontend project</h1>
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
