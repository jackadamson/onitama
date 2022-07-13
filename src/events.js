import { EVENT_URL } from './config';

const onEvent = (eventData) => {
  window
    .fetch(EVENT_URL, {
      method: 'POST',
      body: eventData,
      mode: 'no-cors',
    })
    .catch(() => {});
};

export default onEvent;
