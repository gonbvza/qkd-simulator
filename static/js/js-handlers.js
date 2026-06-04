document
  .getElementById('close-panel')
  .addEventListener('click', () => {
    document
      .getElementById('node-panel')
      .classList.remove('open');
  });
