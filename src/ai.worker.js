const onitamaLib = import('./onitamalib');

onmessage = async (e) => {
  const { agentMove } = await onitamaLib;
  const result = agentMove(e.data);
  postMessage(result);
};
