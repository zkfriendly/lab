import { useState } from "react";

function ListGroup() {
  let items = ["Berlin", "Paris", "London", "New York"];

  let [selectedIndex, setSelectedIndex] = useState(-1);

  const handleClick = (e: React.MouseEvent) => {
    console.log(e);
  };

  return (
    <>
      <h1>List Group</h1>
      {items.length === 0 && <p>No items found</p>}
      <ul className="list-group">
        {items.map((item, index) => (
          <li
            onClick={() => setSelectedIndex(index)}
            key={item}
            className={
              selectedIndex === index
                ? "list-group-item active"
                : "list-group-item"
            }
          >
            {item}
          </li>
        ))}
      </ul>
    </>
  );
}

export default ListGroup;
