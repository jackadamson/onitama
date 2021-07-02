import React from 'react';
import { makeStyles } from '@material-ui/core';

const useStyles = makeStyles({
  link: {
    position: 'absolute',
    top: '0px',
    right: '0px',
  },
});
const GithubRibbon = () => {
  const classes = useStyles();
  return (
    <a
      href="https://github.com/jackadamson/onitama"
      className={classes.link}
      target="_blank"
      rel="noreferrer noopener"
    >
      <img
        alt="Fork me on GitHub"
        src="https://github.blog/wp-content/uploads/2008/12/forkme_right_gray_6d6d6d.png?resize=149%2C149"
      />
    </a>
  );
};

export default GithubRibbon;
