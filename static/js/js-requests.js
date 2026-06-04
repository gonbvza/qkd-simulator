const BASE_URL = "http://localhost:3000/api";

async function getState() {
  try {
    const response = await fetch(`${BASE_URL}/state/get`);
    if (!response.ok) throw new Error(`Response status: ${response.status}`);
    return response.json();
  } catch (error) {
    console.error(error.message);
  }
}

async function createNode(nodeType = 'ClientNode') {
  const response = await fetch(`${BASE_URL}/state/node/create`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ name: "client", node_type: nodeType })
  });
  return response;
}

async function createLink(fromId, toId, distance = 1, is_secure) {
  const response = await fetch(`${BASE_URL}/state/link/create`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ src_id: fromId, dst_id: toId, distance, is_secure })
  });
  return response.json();
}
