/* eslint-disable react/prop-types */
import React from 'react';
import PropTypes from 'prop-types';
import ReactMarkdown from 'markdown-to-jsx';
import { withStyles } from '@material-ui/core/styles';
import { Typography, Link } from '@material-ui/core';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faChessQueen, faChessKnight } from '@fortawesome/free-solid-svg-icons';

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

// Custom component for WindSpiritIcon
function WindSpiritIcon({ size = 'medium' }) {
  let iconSize = '2x';
  if (size === 'small') iconSize = '1x';
  if (size === 'large') iconSize = '3x';

  return (
    <span style={{ display: 'inline-block', textAlign: 'center', margin: '8px' }}>
      <FontAwesomeIcon icon={faChessQueen} color="#B5B4B4" size={iconSize} />
    </span>
  );
}

// Custom component for NinjaIcon
function NinjaIcon({ size = 'medium', color = 'red', hidden = false }) {
  let iconSize = '2x';
  if (size === 'small') iconSize = '1x';
  if (size === 'large') iconSize = '3x';

  const colorMap = {
    red: '#f44336',
    blue: '#2196f3',
  };

  const opacity = hidden ? 0.5 : 1;

  return (
    <span style={{ display: 'inline-block', textAlign: 'center', margin: '8px', opacity }}>
      <FontAwesomeIcon icon={faChessKnight} color={colorMap[color] || '#f44336'} size={iconSize} />
    </span>
  );
}

const options = {
  overrides: {
    h1: { component: Typography, props: { gutterBottom: true, variant: 'h4' } },
    h2: { component: Typography, props: { gutterBottom: true, variant: 'h5' } },
    h3: { component: Typography, props: { gutterBottom: true, variant: 'h6' } },
    h4: {
      component: Typography,
      props: { gutterBottom: true, variant: 'caption', paragraph: true },
    },
    p: { component: Typography, props: { paragraph: true } },
    a: { component: Link },
    img: {
      component: withStyles(styles)(({ classes, alt, src }) => (
        <img alt={alt || ''} src={src} className={classes.image} />
      )),
    },
    li: {
      component: withStyles(styles)(({ classes, children }) => (
        <li className={classes.listItem}>
          <Typography component="span">{children}</Typography>
        </li>
      )),
    },
    WindSpiritIcon: { component: WindSpiritIcon },
    NinjaIcon: { component: NinjaIcon },
  },
};

function Markdown({ children }) {
  const renderContent = () => {
    // Process placeholders in the raw content
    const processedContent = children
      .replace(
        /\[WindSpiritIcon(?: size="(small|medium|large)")?\]/g,
        (_, size = 'medium') => `<WindSpiritIcon size="${size}" />`,
      )
      .replace(
        /\[NinjaIcon(?: size="(small|medium|large)")?(?: color="(red|blue)")?(?: hidden)?\]/g,
        (_, size = 'medium', color = 'red') =>
          `<NinjaIcon size="${size}" color="${color}" hidden="${!!_.includes('hidden')}" />`,
      );

    return (
      <ReactMarkdown
        options={{
          ...options,
        }}
      >
        {processedContent}
      </ReactMarkdown>
    );
  };

  return renderContent();
}

Markdown.propTypes = {
  children: PropTypes.string.isRequired,
};

export default Markdown;
