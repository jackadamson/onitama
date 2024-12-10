import React from 'react';
import { Link, useParams } from 'react-router-dom';
import { Box, Button } from '@material-ui/core';
import Markdown from './Markdown';
import rulesDataSet from './rulesData';

function Rules() {
  const { page } = useParams();
  const rule = rulesDataSet.find((r) => r.path.endsWith(page));
  const content = rule?.content || '# Page Not Found';

  return (
    <Box m={2}>
      {/* Top Navigation Buttons */}
      <Box display="flex" justifyContent="flex-start" mb={2} flexWrap="wrap" maxWidth="720px" width="100%" mx="auto">
        {/* Back to Menu Button */}
        <Box mx={1} mb={1}>
          <Button
            component={Link}
            to="/"
            variant="outlined"
            color="secondary"
          >
            Back to Menu
          </Button>
        </Box>
        {/* Navigation Buttons */}
        {rulesDataSet.map((r) => (
          <Box key={r.path} mx={1} mb={1}>
            <Button
              component={Link}
              to={r.path}
              variant={r.path.endsWith(page) ? 'contained' : 'outlined'} // Highlight current page
              color={r.path.endsWith(page) ? 'primary' : 'default'} // Change color for current page
            >
              {r.label}
            </Button>
          </Box>
        ))}
      </Box>

      {/* Rules Content */}
      <Box display="flex" alignItems="center" justifyContent="center">
        <Box maxWidth="720px" width="100%">
          <Markdown>{content}</Markdown>
          {/* Bottom Buttons */}
          <Box display="flex" mt={2}>
            <Button
              variant="outlined"
              color="secondary"
              component={Link}
              to="/"
            >
              Back to Menu
            </Button>
            <Box flexGrow={1} />
            {/* Conditionally Render Play an Easy Game Button */}
            {page === 'base-game' && (
              <Button
                variant="contained"
                color="primary"
                component={Link}
                to="/ai/easy"
              >
                Play an Easy Game
              </Button>
            )}
          </Box>
        </Box>
      </Box>
    </Box>
  );
}

export default Rules;
