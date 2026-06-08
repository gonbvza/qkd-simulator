let cy;
let pendingNode = null;
let pendingLink = null;
let linkStartNode = null;
let isDragging = false;

function openPanel() {
  document.getElementById('node-panel').classList.add('open');
}

function closePanel() {
  document.getElementById('node-panel').classList.remove('open');
  pendingNode = null;
  pendingLink = null;
  linkStartNode = null;
  hideAllFields();
}

function hideAllFields() {
  document.querySelectorAll('.field').forEach(el => el.classList.remove('visible'));
}

function showField(id) {
  document.getElementById(id).classList.add('visible');
}

async function init() {
  const state = await getState();

  console.log(state);
  cy = cytoscape({
    container: document.getElementById('graph'),

    elements: [
      ...state.nodes.map(n => ({
        data: { id: n.id, node_type: n.node_type },
        classes: n.node_type
      })),
      ...state.links.map(e => ({
        data: {
          id: `${e.src_id}-${e.dst_id}`,
          source: e.src_id,
          target: e.dst_id,
          distance: e.distance,
          is_secure: String(e.is_secure)
        }
      }))
    ],

    style: [
      // ================= Nodes =================
      {
        selector: 'node',
        style: {
          label: 'data(id)',
          'text-valign': 'center',
          color: '#111'
        }
      },
      { selector: '.ClientNode', style: { 'background-color': '#2F80ED' } },
      { selector: '.EprNode', style: { 'background-color': '#9B51E0' } },

      // ================= Edges (base style) =================
      {
        selector: 'edge',
        style: {
          width: 3,
          'curve-style': 'bezier',
          'target-arrow-shape': 'none',

          label: 'data(distance)',
          'font-size': 10,
          'text-rotation': 'autorotate',
          'text-margin-y': -10,

          color: '#333',
          'text-background-color': '#ffffff',
          'text-background-opacity': 0.7,
          'text-background-padding': '2px',

          'line-color': '#bbb',
          'target-arrow-color': '#bbb'
        }
      },

      // ================= Insecure edges (RED) =================
      {
        selector: 'edge[is_secure = "false"]',
        style: {
          'line-color': '#ef4444',
          'target-arrow-color': '#ef4444'
        }
      },

      // ================= Link selection highlight =================
      {
        selector: '.link-source',
        style: {
          'border-width': 3,
          'border-color': '#f90',
          'border-style': 'solid'
        }
      }
    ],

    layout: { name: 'grid' }
  });

  cy.on('dragstart', 'node', () => { isDragging = true; });
  cy.on('dragstop', 'node', () => {
    setTimeout(() => { isDragging = false; }, 100);
  });

  /* -------------------------
     Canvas tap → create node
  -------------------------- */
  cy.on('tap', function(event) {
    if (event.target !== cy) return;
    if (isDragging) return;

    const pos = event.position;
    const tempId = `temp-node-${Date.now()}`;

    const node = cy.add({
      group: 'nodes',
      data: { id: tempId, node_type: 'ClientNode' },
      classes: 'ClientNode',
      position: { x: pos.x, y: pos.y }
    });

    node.lock();
    setTimeout(() => node.unlock(), 300);

    pendingNode = node;
    pendingLink = null;

    hideAllFields();
    document.getElementById('node-temp-id').textContent = tempId;
    document.getElementById('node-type-input').value = 'ClientNode';
    showField('field-node-id');
    showField('field-node-type');
    openPanel();
  });

  /* -------------------------
     Node tap → inspect or link
  -------------------------- */
  cy.on('tap', 'node', function(event) {
    event.stopPropagation();
    if (isDragging) return;

    const node = event.target;
    const isCtrl = event.originalEvent?.ctrlKey || event.originalEvent?.metaKey;

    if (isCtrl) {
      if (!linkStartNode) {
        linkStartNode = node;
        node.addClass('link-source');
        return;
      }

      if (linkStartNode.id() === node.id()) {
        linkStartNode.removeClass('link-source');
        linkStartNode = null;
        return;
      }

      const tempEdgeId = `temp-edge-${Date.now()}`;

      cy.add({
        group: 'edges',
        data: {
          id: tempEdgeId,
          source: linkStartNode.id(),
          target: node.id(),
          distance: 1,
          is_secure: true
        }
      });

      pendingLink = {
        id: tempEdgeId,
        from: linkStartNode.id(),
        to: node.id()
      };

      pendingNode = null;

      hideAllFields();
      document.getElementById('link-from').textContent = linkStartNode.id();
      document.getElementById('link-to').textContent = node.id();
      document.getElementById('link-distance-input').value = 1;
      document.getElementById('link-secure-input').checked = true;

      showField('field-link-from');
      showField('field-link-to');
      showField('field-link-distance');
      showField('field-link-secure');
      openPanel();

      linkStartNode.removeClass('link-source');
      linkStartNode = null;
      return;
    }

    /* Normal click */
    if (linkStartNode) {
      linkStartNode.removeClass('link-source');
      linkStartNode = null;
    }

    pendingNode = node;
    pendingLink = null;

    hideAllFields();
    document.getElementById('node-temp-id').textContent = node.id();
    document.getElementById('node-type-input').value = node.data('node_type');
    showField('field-node-id');
    showField('field-node-type');
    openPanel();
  });

  /* -------------------------
     Edge tap → inspect
  -------------------------- */
  cy.on('tap', 'edge', function(event) {
    event.stopPropagation();
    const edge = event.target;

    pendingLink = {
      id: edge.id(),
      from: edge.source().id(),
      to: edge.target().id()
    };

    pendingNode = null;

    hideAllFields();
    document.getElementById('link-from').textContent = edge.source().id();
    document.getElementById('link-to').textContent = edge.target().id();
    document.getElementById('link-distance-input').value = edge.data('distance') ?? 1;
    document.getElementById('link-secure-input').checked = edge.data('is_secure');

    showField('field-link-from');
    showField('field-link-to');
    showField('field-link-distance');
    showField('field-link-secure');
    openPanel();
  });

  document.getElementById('close-panel')
    .addEventListener('click', closePanel);

  /* -------------------------
     Save
  -------------------------- */
  document.getElementById('save-btn').addEventListener('click', async () => {

    if (pendingNode && pendingNode.id().startsWith('temp-node-')) {
      const nodeType = document.getElementById('node-type-input').value;
      await createNode(nodeType);
      pendingNode = null;
    }

    if (pendingLink && pendingLink.id.startsWith('temp-edge-')) {
      const distance = Number(document.getElementById('link-distance-input').value);
      const is_secure = document.getElementById('link-secure-input').checked;

      await createLink(pendingLink.from, pendingLink.to, distance, is_secure);

      const edge = cy.getElementById(pendingLink.id);
      edge.data('distance', distance);
      edge.data('is_secure', is_secure);

      pendingLink = null;
    }

    closePanel();
  });
}

init();
