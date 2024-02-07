function ListGroup() {
  let items = ["Berlin", "Paris", "London", "New York"];

  if (items.length === 0) {
    return <p>No items found</p>;
  }

  const handleClick = (e: React.MouseEvent) => {
    console.log(e);
  };

  return (
    <>
      <h1>List Group</h1>
      {items.length === 0 && <p>No items found</p>}
      <ul className="list-group">
        {items.map((item) => (
          <li onClick={handleClick} key={item} className="list-group-item">
            {item}
          </li>
        ))}
      </ul>
    </>
  );
}

export default ListGroup;
