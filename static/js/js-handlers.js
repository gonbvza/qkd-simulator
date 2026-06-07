document
  .getElementById('close-panel')
  .addEventListener('click', () => {
    document
      .getElementById('node-panel')
      .classList.remove('open');
  });

let socket = new WebSocket("ws://localhost:3000/ws");

socket.onopen = function(e) {
  alert("[open] Connection established");
  alert("Sending to server");
  socket.send("My name is John");
};

socket.onmessage = function(event) {
  alert(`[message] Data received from server: ${event.data}`);
};
