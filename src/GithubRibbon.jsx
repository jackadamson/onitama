import React from 'react';
import { makeStyles } from '@material-ui/core';
import githubRibbonImage from './resources/forkme_right_gray.png';

const useStyles = makeStyles({
  link: {
    position: 'absolute',
    top: '0px',
    right: '0px',
  },
});
function GithubRibbon() {
  const classes = useStyles();
  return (
    <a
      href="https://github.com/jackadamson/onitama"
      className={classes.link}
      target="_blank"
      rel="noreferrer noopener"
    >
      <img alt="Fork me on GitHub" src={githubRibbonImage} />
    </a>
  );
}

export default GithubRibbon;
