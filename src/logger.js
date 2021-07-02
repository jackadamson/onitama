const noop = () => {};

const fakeLogger = Object.fromEntries(Object.keys(console).map((name) => [name, noop]));

const logger = process.env.NODE_ENV === 'production' ? fakeLogger : console;

export default logger;
