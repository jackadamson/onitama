let meta = null;

function getMetaUncached() {
  const build = process.env.REACT_APP_BUILD_VERSION || 'unknown';
  const existingUid = localStorage.getItem('uid');
  if (existingUid != null) {
    return { build, uid: existingUid };
  }
  const newUid = window.crypto.randomUUID();
  localStorage.setItem('uid', newUid);
  return { build, uid: newUid };
}
export default function getMeta() {
  // get non-identifying metadata to log to server for troubleshooting
  if (meta != null) return meta;

  try {
    meta = getMetaUncached();
  } catch (e) {
    // TODO: handle
  }

  return meta;
}
