const protocolTranslate = {
  'http:': 'ws',
  'https:': 'wss',
};
const websocketProtocol = protocolTranslate[document.location.protocol];
export const PRODUCTION_WEBSOCKET_BASE = `${websocketProtocol}://${document.location.host}/ws/`;

export const WEBSOCKET_BASE =
  process.env.NODE_ENV === 'development' ? 'ws://localhost:8080/ws/' : PRODUCTION_WEBSOCKET_BASE;
