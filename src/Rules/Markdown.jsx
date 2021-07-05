/* eslint-disable react/jsx-props-no-spreading */
import React from 'react';
import PropTypes from 'prop-types';
import ReactMarkdown from 'markdown-to-jsx';
import { withStyles } from '@material-ui/core/styles';
import { Typography, Link } from '@material-ui/core';

const styles = (theme) => ({
  listItem: {
    marginTop: theme.spacing(1),
  },
  image: {
    objectFit: 'cover',
    width: '50%',
    display: 'block',
    marginLeft: 'auto',
    marginRight: 'auto',
  },
});

const options = {
  overrides: {
    h1: {
      component: Typography,
      props: {
        gutterBottom: true,
        variant: 'h4',
      },
    },
    h2: { component: Typography, props: { gutterBottom: true, variant: 'h5' } },
    h3: { component: Typography, props: { gutterBottom: true, variant: 'h6' } },
    h4: {
      component: Typography,
      props: { gutterBottom: true, variant: 'caption', paragraph: true },
    },
    p: { component: Typography, props: { paragraph: true } },
    a: { component: Link },
    img: {
      component: withStyles(styles)(({ classes, alt, ...props }) => (
        <img alt="alt" {...props} className={classes.image} />
      )),
    },
    li: {
      component: withStyles(styles)(({ classes, ...props }) => (
        <li className={classes.listItem}>
          <Typography component="span" {...props} />
        </li>
      )),
    },
  },
};

const Markdown = ({ children }) => <ReactMarkdown options={options}>{children}</ReactMarkdown>;
Markdown.propTypes = {
  children: PropTypes.string.isRequired,
};
export default Markdown;
