import ListGroup from "./components/ListGroup";

function App() {
  let items = ["Berlin", "Paris", "London", "New York"];
  return (
    <div>
      <ListGroup items={items} heading="Cities" />
    </div>
  );
}

export default App;
