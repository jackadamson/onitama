import boardScreenshot from './screenshots/board.png';
import cardScreenshot from './screenshots/card.png';
import WindMoveCardScreenshot from './screenshots/wind-move-card.png';

export const rulesData = [
  {
    path: '/rules/base-game',
    label: 'Base Game',
    content: `
# Onitama

Onitama is an elegant and simple game which captures the essence of martial arts.

Each game is quick, usually 15 minutes in length.

## Board

![Onitama Board](${boardScreenshot})

Each game will use 5 cards chosen randomly from the 16 game cards.

The board begins with each player controlling **4 pawns** and **1 king**.

Each player has a hand of **2 cards**.

One card is a spare on the side of the board (or on mobile highlighted in the hand).

## Playing the Game

Players alternate turns until the game has ended.

On a player's turn, they select one of their two cards, determining where they can move one of their pieces.

![Ox Card](${cardScreenshot})

For example, the Ox card lets the player move a piece either:

- One square forward
- One square right
- One square behind.

**Note: Card moves are relative to the player's perspective, so the card will rotate 180 degrees when the opponent has it.**

Pieces can be moved to any square that is not occupied by a same-colored piece.

The played card then switches with the spare card.

### Capture

If a piece is moved to the same square as an opponent's piece, the opponent's piece is **captured** and removed from the game.

### Discarding

If you have no moves that you can legally make, you instead will discard a card, playing it and not moving a piece.

If you can play a move, you cannot discard.

## Winning the Game

The game can be won in one of two ways:

### King Capture

If a player **captures** an opponent's **king**, they win the game.

### Base Capture

If a player moves their **king** to the **starting square** of the opponent (the colored square), they win the game.
    `,
  },
  {
    path: '/rules/way-of-the-wind',
    label: 'Way of the Wind',
    content: `
# Way of the Wind

In the Way of the Wind expansion, a neutral Wind Spirit piece and new special move cards are used.

## Setup

The board and player pieces are set up the same as in the base game.

In the center square, there is a new neutral piece called the **Wind Spirit**. 

<div style="text-align: center; margin: 16px 0;">
  [WindSpiritIcon size="large"]
</div>

The game uses 5 cards as normal, but each player will start with one Wind Spirit move card.

![Wind Spirit Move Card Example](${WindMoveCardScreenshot})

## Playing the Game

Play proceeds as in the normal game with these changes:

- When using a regular move card, you may use it to move one of your pieces or the **Wind Spirit**[WindSpiritIcon size="small"]
- If you use a Wind Spirit move card to move, first you move one of your pieces according to the top grid, then you **must** move the Wind Spirit (if able) using one of the moves on the bottom grid.

![Wind Spirit Move Card Example](${WindMoveCardScreenshot})

## Moving the Wind Spirit

The **Wind Spirit** moves like any other piece when using a move card. 

- If the Wind Spirit lands on a Pawn (friendly or enemy), it does not capture it. Instead, they **swap places**.
- The Wind Spirit cannot move into the same space as a King.
- No piece can move onto the Wind Spirit, effectively blocking its square.

## Alternate Setup

You can adjust the setup for the Wind Spirit game using the settings menu.

- This allows you to force the Wind Spirit into every game.
- You can also adjust the number of Wind Spirit move cards in the game.
    `,
  },
];

export default rulesData;
