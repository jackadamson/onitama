import React from 'react';
import { Link } from 'react-router-dom';
import { Box, Button } from '@material-ui/core';
import Markdown from './Markdown';
import boardScreenshot from './screenshots/board.png';
import cardScreenshot from './screenshots/card.png';

const rules = `
# Onitama

Onitama is an elegant and simple game which captures the essence of martial arts

Each game is quick, usually 15 minutes in length


## Board

![Onitama Board](${boardScreenshot})

Each game will use 5 cards chosen randomly from the 16 game cards

The board begin's with each player controlling **4 pawns** and **1 king**

Each player has a hand of **2 cards**

One card is a spare on the side of the board (or on mobile highlighted in the hand)

## Playing the Game

Players alternate turns until the game has ended

On a player's turn, they select one of their two cards, determining where they can move one of their pieces

![Ox Card](${cardScreenshot})

For example, the Ox card lets the player move a piece either

- One square forward
- One square right
- One square behind 

**Note: Card moves are relative to the player's perspective, so the card will rotate 180 degrees when the opponent has it**

Pieces can be moved to any square that is not occupied by a same coloured piece

The played card then switches with the spare card

### Capture

If a piece is moved to the same square as an opponent's piece, the opponent's piece is **captured** and removed from the game

### Discarding

If you have no moves that you can legally make, you instead will discard a card, playing it and not moving a pieces

If you can play a move, you cannot discard

## Winning the Game

The game can be won in one of two ways

### King Capture

If a player **captures** an opponent's **king**, they win the game

### Base Capture

If a player moves their **king** to the **starting square** of the opponent (the coloured square), they win the game

`;

function Rules() {
  return (
    <Box m={2}>
      <Button component={Link} to="/" variant="outlined">
        Back
      </Button>
      <Box display="flex" alignItems="center" justifyContent="center">
        <Box maxWidth="720px" width="100%">
          <Markdown>{rules}</Markdown>
          <Box display="flex">
            <Button variant="outlined" color="secondary" component={Link} to="/">
              Back to Menu
            </Button>
            <Box flexGrow={1} />
            <Button variant="contained" color="primary" component={Link} to="/ai/easy">
              Play an Easy Game
            </Button>
          </Box>
        </Box>
      </Box>
    </Box>
  );
}

export default Rules;
