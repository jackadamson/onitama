const onitamaLib = import('./onitamalib');

onmessage = async (e) => {
  const { rankMoves } = await onitamaLib;
  const result = rankMoves(e.data);
  postMessage(result);
};
