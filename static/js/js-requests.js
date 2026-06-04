const URL = "http://localhost:8000/api";

async function getState() {
  try {
    const response = await fetch(`${URL}/state/`);
    if (!response.ok) {
      throw new Error(`Response status: ${response.status}`);
    }

    const result = await response.json();
    console.log(result);
  } catch (error) {
    console.error(error.message);
  }
}
