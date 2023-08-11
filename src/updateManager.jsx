import React, {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useRef,
  useState,
} from 'react';
import PropTypes from 'prop-types';
import logger from './logger';
import { registerValidSW } from './serviceWorkerRegistration';

export const UpdateContext = createContext({ updateAvailable: false });
export function UpdateManager({ children }) {
  const swRef = useRef(null);
  const [updateAvailable, setUpdateAvailable] = useState(false);
  const checkUpdate = useCallback(() => {
    logger.log('Check Update Called');
    if (swRef.current) {
      swRef.current.update();
    }
  }, [swRef]);
  useEffect(() => {
    let mounted = true;
    const swUrl = `${process.env.PUBLIC_URL}/service-worker.js`;
    const onUpdate = () => {
      if (mounted) {
        logger.log('Update available');
        setUpdateAvailable(true);
      } else {
        logger.log('Update available but unmounted');
      }
    };
    registerValidSW(swUrl, { onUpdate }).then((sw) => {
      if (mounted) {
        swRef.current = sw;
      }
    });
    return () => {
      mounted = false;
    };
  }, [setUpdateAvailable]);
  const value = useMemo(
    () => ({
      checkUpdate,
      updateAvailable,
    }),
    [checkUpdate, updateAvailable],
  );
  return <UpdateContext.Provider value={value}>{children}</UpdateContext.Provider>;
}

UpdateManager.propTypes = {
  children: PropTypes.element.isRequired,
};

export function useAppUpdater() {
  const { updateAvailable, checkUpdate } = useContext(UpdateContext);
  useEffect(() => {
    if (updateAvailable) {
      window.location.reload();
    }
  }, [updateAvailable]);

  useEffect(() => {
    checkUpdate();
  }, [checkUpdate]);

  return checkUpdate;
}
